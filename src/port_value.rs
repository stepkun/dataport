// Copyright © 2025 Stephan Kunz
//! Internal port value representation and its read & write guards.

use core::ops::{Deref, DerefMut};

use alloc::sync::Arc;

use crate::{
	ConstString, RwLock, RwLockReadGuard, RwLockWriteGuard,
	error::{Error, Result},
	sequence_number::SequenceNumber,
};

/// Type definition for a pointer to a [`PortValue`]
pub(crate) type PortValuePtr<T> = Arc<RwLock<PortValue<T>>>;

/// Internal representation of a ports value.
/// The data `T` together with its [`SequenceNumber`].
/// The `PortValue` is shared between the bound ports.
pub(crate) struct PortValue<T>(Option<T>, SequenceNumber);

impl<T> Default for PortValue<T> {
	fn default() -> Self {
		Self(None, SequenceNumber::default())
	}
}

impl<T: core::fmt::Debug> core::fmt::Debug for PortValue<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("PortValue")
			.field(&self.0)
			.field(&self.1)
			.finish()
	}
}

impl<T> PortValue<T> {
	pub(crate) fn new(value: impl Into<T>) -> Self {
		let mut sq = SequenceNumber::default();
		sq.increment();
		Self(Some(value.into()), sq)
	}

	pub(crate) const fn as_ref(&self) -> Option<&T> {
		self.0.as_ref()
	}

	pub(crate) const fn is_some(&self) -> bool {
		self.0.is_some()
	}

	pub(crate) fn is_none(&self) -> bool {
		self.0.is_none()
	}

	pub(crate) fn replace(&mut self, value: impl Into<T>) -> Option<T> {
		self.1.increment();
		self.0.replace(value.into())
	}

	pub(crate) const fn sequence_number(&self) -> u32 {
		self.1.value()
	}

	pub(crate) fn set(&mut self, value: impl Into<T>) {
		self.1.increment();
		self.0 = Some(value.into())
	}

	pub(crate) fn take(&mut self) -> Option<T> {
		self.1.increment();
		self.0.take()
	}
}

impl<T: Clone> PortValue<T> {
	pub(crate) fn get(&self) -> Option<T> {
		self.0.clone()
	}
}

/// Read-Locked port value guard.
/// Until this value is dropped, a read lock is held on the ports value.
///
/// Implements [`Deref`], providing read access to the locked `T`.
#[must_use = "a `PortValueReadGuard` should be used"]
pub struct PortValueReadGuard<T> {
	/// `Arc` to a `value`
	value: PortValuePtr<T>,
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

impl<T> PortValueReadGuard<T> {
	/// Returns a read guard to a T.
	/// # Errors
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub(crate) fn new(port: impl Into<ConstString>, value: PortValuePtr<T>) -> Result<Self> {
		// we know this pointer is valid since the guard owns the value
		let ptr_t = {
			let guard = value.read();
			// leak returns &'rwlock &Option<T> but read locks RwLock forewer
			let x = RwLockReadGuard::leak(guard);
			if let Some(value) = &x.0 {
				let ptr_t: *const T = value;
				ptr_t
			} else {
				return Err(Error::ValueNotSet { port: port.into() });
			}
		};

		Ok(Self { value, ptr_t })
	}

	/// Returns a read guard to a T.
	/// # Errors
	/// - [`Error::IsLocked`]  if the entry is locked by someone else.
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub(crate) fn try_new(port: impl Into<ConstString>, value: PortValuePtr<T>) -> Result<Self> {
		// we know this pointer is valid since the guard owns the value
		let ptr_t = {
			if let Some(guard) = value.try_read() {
				// leak returns &'rwlock &Option<T> but read locks RwLock forewer
				let x = RwLockReadGuard::leak(guard);
				if let Some(value) = &x.0 {
					let ptr_t: *const T = value;
					ptr_t
				} else {
					return Err(Error::ValueNotSet { port: port.into() });
				}
			} else {
				return Err(Error::IsLocked { port: port.into() });
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
	value: PortValuePtr<T>,
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

impl<T> PortValueWriteGuard<T> {
	/// Returns a write guard to a T.
	/// # Errors
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub(crate) fn new(port: impl Into<ConstString>, value: PortValuePtr<T>) -> Result<Self> {
		// we know this pointer is valid since the guard owns the value
		let (ptr_t, ptr_seq_id) = {
			let guard = value.write();
			// leak returns &'rwlock &Option<T> but write locks RwLock forewer
			let x = RwLockWriteGuard::leak(guard);
			if let Some(value) = &mut x.0 {
				let ptr_t: *mut T = value;
				let ptr_seq_id: *mut SequenceNumber = &raw mut x.1;
				(ptr_t, ptr_seq_id)
			} else {
				return Err(Error::ValueNotSet { port: port.into() });
			}
		};

		Ok(Self {
			value,
			ptr_t,
			ptr_seq_id,
			modified: false,
		})
	}

	/// Returns a write guard to a T.
	/// # Errors
	/// - [`Error::IsLocked`]  if the entry is locked by someone else.
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub(crate) fn try_new(port: impl Into<ConstString>, value: PortValuePtr<T>) -> Result<Self> {
		// we know this pointer is valid since the guard owns the value
		let (ptr_t, ptr_seq_id) = {
			if let Some(guard) = value.try_write() {
				// leak returns &'rwlock &Option<T> but write locks RwLock forewer
				let x = RwLockWriteGuard::leak(guard);
				if let Some(value) = &mut x.0 {
					let ptr_t: *mut T = value;
					let ptr_seq_id: *mut SequenceNumber = &raw mut x.1;
					(ptr_t, ptr_seq_id)
				} else {
					return Err(Error::ValueNotSet { port: port.into() });
				}
			} else {
				return Err(Error::IsLocked { port: port.into() });
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
