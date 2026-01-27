// Copyright © 2026 Stephan Kunz
//! Internal port value representation and its read & write guards.

use core::ops::{Deref, DerefMut};

use alloc::{boxed::Box, sync::Arc};

use crate::{
	RwLock, RwLockReadGuard, RwLockWriteGuard, any_port_value::AnyPortValue, bind::sequence_number::SequenceNumber,
	error::Error,
};

/// Type definition for a pointer to a [`PortValue`]
pub(crate) type PortValuePtr = Arc<RwLock<(Box<dyn AnyPortValue>, SequenceNumber)>>;

/// Internal representation of a ports value.
/// The `PortValue` is shared between the bound ports.
#[derive(Default)]
#[repr(transparent)]
pub(crate) struct PortValue<T>(Option<T>);

impl<T> PortValue<T> {
	pub(crate) fn empty() -> Self {
		Self(None)
	}

	pub(crate) fn new(value: T) -> Self {
		Self(Some(value))
	}

	pub(crate) fn replace(&mut self, value: T) -> Option<T> {
		self.0.replace(value)
	}

	pub(crate) fn set(&mut self, value: T) {
		self.0 = Some(value)
	}

	pub(crate) fn take(&mut self) -> Option<T> {
		self.0.take()
	}
}

impl<T: Clone> PortValue<T> {
	pub(crate) fn get(&self) -> Option<T> {
		self.0.clone()
	}
}

impl<T: core::fmt::Debug> core::fmt::Debug for PortValue<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("PortValue").field(&self.0).finish()
	}
}

/// Read-Locked port value guard.
/// Until this value is dropped, a read lock is held on the ports value.
///
/// Implements [`Deref`], providing read access to the locked `T`.
#[must_use = "a `PortValueReadGuard` should be used"]
pub struct PortValueReadGuard<T> {
	/// `Arc` to a `value`
	value: PortValuePtr,
	/// Immutable pointer to content of the `value` above
	ptr_t: *const T,
}

impl<T> Deref for PortValueReadGuard<T> {
	type Target = T;

	#[allow(unsafe_code)]
	fn deref(&self) -> &Self::Target {
		// SAFETY: Self referencing to locked content of the `Arc` `Entry`, valid until self is dropped
		unsafe { &*self.ptr_t }
	}
}

impl<T> Drop for PortValueReadGuard<T> {
	#[allow(unsafe_code)]
	fn drop(&mut self) {
		// SAFETY: manually decrementing lock because entry is permanently locked in new()
		unsafe {
			self.value.force_read_decrement();
		}
	}
}

impl<T: 'static> PortValueReadGuard<T> {
	/// Returns a read guard to the T.
	/// # Errors
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub(crate) fn new(value: PortValuePtr) -> Result<Self, Error> {
		// we know this pointer is valid since the guard owns the value
		let ptr_t = {
			let guard = value.read();
			// SAFETY: leak returns &'rwlock &Option<T> but locks RwLock forewer.
			// We have to take care of unlocking manually.
			let x = RwLockReadGuard::leak(guard);
			if let Some(v) =
				x.0.as_ref()
					.as_any()
					.downcast_ref::<PortValue<T>>()
			{
				if let Some(value) = &v.0 {
					let ptr_t: *const T = value;
					ptr_t
				} else {
					// SAFETY: Remove the lock
					unsafe {
						value.force_read_decrement();
					};
					return Err(Error::NoValueSet);
				}
			} else {
				// SAFETY: Remove the lock
				unsafe {
					value.force_read_decrement();
				};
				return Err(Error::DataType);
			}
		};

		Ok(Self { value, ptr_t })
	}

	/// Returns a read guard to the T.
	/// # Errors
	/// - [`Error::IsLocked`]  if the entry is locked by someone else.
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub(crate) fn try_new(value: PortValuePtr) -> Result<Self, Error> {
		// we know this pointer is valid since the guard owns the value
		let ptr_t = {
			if let Some(guard) = value.try_read() {
				// SAFETY: leak returns &'rwlock &Option<T> but locks RwLock forewer.
				// We have to take care of unlocking manually.
				let x = RwLockReadGuard::leak(guard);
				if let Some(v) =
					x.0.as_ref()
						.as_any()
						.downcast_ref::<PortValue<T>>()
				{
					if let Some(value) = &v.0 {
						let ptr_t: *const T = value;
						ptr_t
					} else {
						// SAFETY: Remove the lock
						unsafe {
							value.force_read_decrement();
						};
						return Err(Error::NoValueSet);
					}
				} else {
					// SAFETY: Remove the lock
					unsafe {
						value.force_read_decrement();
					};
					return Err(Error::DataType);
				}
			} else {
				return Err(Error::IsLocked);
			}
		};

		Ok(Self { value, ptr_t })
	}
}

/// Write-Locked port value guard.
/// Until this value is dropped, a write lock is held on the ports value.
///
/// Implements [`Deref`] & [`DerefMut`], providing access to the locked `T`.
#[must_use = "a `PortValueWriteGuard` should be used"]
pub struct PortValueWriteGuard<T> {
	/// `Arc` to a `value`.
	value: PortValuePtr,
	/// Mutable pointer to content of the `value` above.
	ptr_t: *mut T,
	/// Mutable pointer to the sequence_id.
	ptr_seq_id: *mut SequenceNumber,
	/// Change flag.
	modified: bool,
}

impl<T> Deref for PortValueWriteGuard<T> {
	type Target = T;

	#[allow(unsafe_code)]
	fn deref(&self) -> &Self::Target {
		// SAFETY: Self referencing to locked content of the `Arc` `Entry`, valid until self is dropped
		unsafe { &*self.ptr_t }
	}
}

impl<T> DerefMut for PortValueWriteGuard<T> {
	#[allow(unsafe_code)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		// once dereferenced mutable we assume a modification
		self.modified = true;
		// SAFETY: Self referencing to locked content of the `Arc` `Entry`, valid until self is dropped
		unsafe { &mut *self.ptr_t }
	}
}

impl<T> Drop for PortValueWriteGuard<T> {
	#[allow(unsafe_code)]
	fn drop(&mut self) {
		// SAFETY: manually removing lock because entry is permanently locked in new()
		unsafe {
			// if modified, increment sequence id
			if self.modified {
				self.ptr_seq_id.as_mut().unwrap().increment();
			}

			self.value.force_write_unlock();
		}
	}
}

impl<T: 'static> PortValueWriteGuard<T> {
	/// Returns a write guard to the T.
	/// # Errors
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub(crate) fn new(value: PortValuePtr) -> Result<Self, Error> {
		// we know this pointer is valid since the guard owns the value
		let (ptr_t, ptr_seq_id) = {
			let guard = value.write();
			// SAFETY: leak returns &'rwlock &Option<T> but locks RwLock forewer.
			// We have to take care of unlocking manually.
			let x = RwLockWriteGuard::leak(guard);
			if let Some(v) =
				x.0.as_mut()
					.as_mut_any()
					.downcast_mut::<PortValue<T>>()
			{
				if let Some(value) = &mut v.0 {
					let ptr_t: *mut T = value;
					let ptr_seq_id: *mut SequenceNumber = &raw mut x.1;
					(ptr_t, ptr_seq_id)
				} else {
					// SAFETY: Remove the lock
					unsafe {
						value.force_write_unlock();
					};
					return Err(Error::NoValueSet);
				}
			} else {
				// SAFETY: Remove the lock
				unsafe {
					value.force_write_unlock();
				};
				return Err(Error::DataType);
			}
		};

		Ok(Self {
			value,
			ptr_t,
			ptr_seq_id,
			modified: false,
		})
	}

	/// Returns a write guard to the T.
	/// # Errors
	/// - [`Error::IsLocked`]  if the entry is locked by someone else.
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub(crate) fn try_new(value: PortValuePtr) -> Result<Self, Error> {
		// we know this pointer is valid since the guard owns the value
		let (ptr_t, ptr_seq_id) = {
			if let Some(guard) = value.try_write() {
				// SAFETY: leak returns &'rwlock &Option<T> but locks RwLock forewer.
				// We have to take care of unlocking manually.
				let x = RwLockWriteGuard::leak(guard);
				if let Some(v) =
					x.0.as_mut()
						.as_mut_any()
						.downcast_mut::<PortValue<T>>()
				{
					if let Some(value) = &mut v.0 {
						let ptr_t: *mut T = value;
						let ptr_seq_id: *mut SequenceNumber = &raw mut x.1;
						(ptr_t, ptr_seq_id)
					} else {
						// SAFETY: Remove the lock
						unsafe {
							value.force_write_unlock();
						};
						return Err(Error::NoValueSet);
					}
				} else {
					// SAFETY: Remove the lock
					unsafe {
						value.force_write_unlock();
					};
					return Err(Error::DataType);
				}
			} else {
				return Err(Error::IsLocked);
			}
		};

		Ok(Self {
			value,
			ptr_t,
			ptr_seq_id,
			modified: false,
		})
	}
}

#[cfg(test)]
mod tests {
	use alloc::string::String;

	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&PortValue<i32>>();
		is_normal::<PortValue<String>>();
	}
}
