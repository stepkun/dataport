// Copyright © 2025 Stephan Kunz
//! Read & write guards.

use core::ops::{Deref, DerefMut};

use alloc::sync::Arc;

use crate::{
	ConstString, RwLock, RwLockReadGuard, RwLockWriteGuard,
	error::{Error, Result},
	out_port::OutputValue,
};

/// Read-Locked port guard.
/// Until this value is dropped, a read lock is held on the ports value.
///
/// Implements [`Deref`], providing read access to the locked `T`.
#[must_use = "a `PortReadGuard` should be used"]
pub struct PortReadGuard<T> {
	/// `Arc` to a `value`
	value: Arc<RwLock<OutputValue<T>>>,
	/// Immutable pointer to content of the `value` above
	ptr_t: *const T,
}

impl<T> Deref for PortReadGuard<T> {
	type Target = T;

	#[allow(unsafe_code)]
	fn deref(&self) -> &Self::Target {
		// SAFETY: Self referencing to locked content of the `Arc` `Entry`, valid until self is dropped
		unsafe { &*self.ptr_t }
	}
}

impl<T> Drop for PortReadGuard<T> {
	#[allow(unsafe_code)]
	fn drop(&mut self) {
		// SAFETY: manually decrementing lock because entry is permanently locked in new()
		unsafe {
			self.value.force_read_decrement();
		}
	}
}

impl<T> PortReadGuard<T> {
	/// Returns a read guard to a &T.
	/// # Errors
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub fn new(port: impl Into<ConstString>, value: Arc<RwLock<OutputValue<T>>>) -> Result<Self> {
		// we know this pointer is valid since the guard owns the value
		let ptr_t = {
			let guard = value.read();
			// leak returns &'rwlock &Option<T> but read locks RwLock forewer
			let x = RwLockReadGuard::leak(guard);
			if let Some(value) = &x.0 {
				let ptr_t: *const T = value;
				ptr_t
			} else {
				return Err(Error::NoValueSet { port: port.into() });
			}
		};

		Ok(Self { value, ptr_t })
	}

	/// Returns a read guard to a &mut T.
	/// # Errors
	/// - [`Error::IsLocked`]  if the entry is locked by someone else.
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub fn try_new(port: impl Into<ConstString>, value: Arc<RwLock<OutputValue<T>>>) -> Result<Self> {
		// we know this pointer is valid since the guard owns the value
		let ptr_t = {
			if let Some(guard) = value.try_read() {
				// leak returns &'rwlock &Option<T> but read locks RwLock forewer
				let x = RwLockReadGuard::leak(guard);
				if let Some(value) = &x.0 {
					let ptr_t: *const T = value;
					ptr_t
				} else {
					return Err(Error::NoValueSet { port: port.into() });
				}
			} else {
				return Err(Error::IsLocked { port: port.into() });
			}
		};

		Ok(Self { value, ptr_t })
	}
}

/// Write-Locked entry guard.
/// Until this value is dropped, a write lock is held on the ports value.
///
/// Implements [`Deref`] & [`DerefMut`], providing access to the locked `T`.
#[must_use = "a `PortWriteGuard` should be used"]
pub struct PortWriteGuard<T> {
	/// `Arc` to a `value`.
	value: Arc<RwLock<OutputValue<T>>>,
	/// Mutable pointer to content of the `value` above.
	ptr_t: *mut T,
	/// Mutable pointer to the sequence is.
	ptr_seq_id: *mut u32,
	/// Change flag.
	modified: bool,
}

impl<T> Deref for PortWriteGuard<T> {
	type Target = T;

	#[allow(unsafe_code)]
	fn deref(&self) -> &Self::Target {
		// SAFETY: Self referencing to locked content of the `Arc` `Entry`, valid until self is dropped
		unsafe { &*self.ptr_t }
	}
}

impl<T> DerefMut for PortWriteGuard<T> {
	#[allow(unsafe_code)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		// once dereferenced mutable we assume a modification
		self.modified = true;
		// SAFETY: Self referencing to locked content of the `Arc` `Entry`, valid until self is dropped
		unsafe { &mut *self.ptr_t }
	}
}

impl<T> Drop for PortWriteGuard<T> {
	#[allow(unsafe_code)]
	fn drop(&mut self) {
		// SAFETY: manually removing lock because entry is permanently locked in new()
		unsafe {
			// if modified, increase sequence id or wrap around to 1
			if self.modified {
				if *self.ptr_seq_id < u32::MAX {
					*self.ptr_seq_id += 1;
				} else {
					*self.ptr_seq_id = 1;
				}
			}

			self.value.force_write_unlock();
		}
	}
}

impl<T> PortWriteGuard<T> {
	/// Returns a write guard to a &mut T.
	/// # Errors
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub fn new(port: impl Into<ConstString>, value: Arc<RwLock<OutputValue<T>>>) -> Result<Self> {
		// we know this pointer is valid since the guard owns the value
		let (ptr_t, ptr_seq_id) = {
			let guard = value.write();
			// leak returns &'rwlock &Option<T> but write locks RwLock forewer
			let x = RwLockWriteGuard::leak(guard);
			if let Some(value) = &mut x.0 {
				let ptr_t: *mut T = value;
				let ptr_seq_id: *mut u32 = &raw mut x.1;
				(ptr_t, ptr_seq_id)
			} else {
				return Err(Error::NoValueSet { port: port.into() });
			}
		};

		Ok(Self {
			value,
			ptr_t,
			ptr_seq_id,
			modified: false,
		})
	}

	/// Returns a write guard to a &mut T.
	/// # Errors
	/// - [`Error::IsLocked`]  if the entry is locked by someone else.
	/// - [`Error::NoValueSet`] if the port does not yet contain a value.
	pub fn try_new(port: impl Into<ConstString>, value: Arc<RwLock<OutputValue<T>>>) -> Result<Self> {
		// we know this pointer is valid since the guard owns the value
		let (ptr_t, ptr_seq_id) = {
			if let Some(guard) = value.try_write() {
				// leak returns &'rwlock &Option<T> but write locks RwLock forewer
				let x = RwLockWriteGuard::leak(guard);
				if let Some(value) = &mut x.0 {
					let ptr_t: *mut T = value;
					let ptr_seq_id: *mut u32 = &raw mut x.1;
					(ptr_t, ptr_seq_id)
				} else {
					return Err(Error::NoValueSet { port: port.into() });
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

//#[cfg(test)]
//mod tests {
//	use super::*;
//
//	const fn is_normal<T: Sized + Send + Sync>() {}
//
//	// check, that the auto traits are available.
//	#[test]
//	const fn normal_types() {
//		is_normal::<PortReadGuard<i32>>();
//		is_normal::<PortWriteGuard<i32>>();
//	}
//}
