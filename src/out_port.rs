// Copyright © 2025 Stephan Kunz
//! OutPort.

#![allow(unused)]

use core::{any::Any, ops::Deref};

use alloc::{boxed::Box, sync::Arc};

use crate::{Error, PortBase, RwLock, RwLockReadGuard, any_extensions::AnySendSync};

/// OutPort
pub struct OutPort<T> {
	/// An identifying name of the port, which must be unique for a given item.
	name: &'static str,
	/// The current value of the port.
	value: RwLock<Option<T>>,
}

impl<T> core::fmt::Debug for OutPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("OutPort")
			.field("name", &self.name)
			//.field("value", &self.value)
			.finish_non_exhaustive()
	}
}

impl<T: 'static> PartialEq for OutPort<T> {
	/// Partial equality of an out port is, if both have the same name & value type
	fn eq(&self, other: &Self) -> bool {
		if self.name == other.name {
			let v1 = self.value.read();
			let v2 = other.value.read();
			if let Some(value1) = &*v1
				&& let Some(value2) = &*v2
			{
				// check type of value1 against type of value2
				if value1.type_id() == value2.type_id() {
					return true;
				}
			} else if v1.is_none() && v2.is_none() {
				return true;
			}
		}
		false
	}
}

impl<T> PortBase for OutPort<T> {
	fn name(&self) -> &'static str {
		self.name
	}
}

impl<T> OutPort<T> {
	#[must_use]
	pub fn new(name: &'static str) -> Self {
		Self {
			name,
			value: RwLock::new(None),
		}
	}

	#[must_use]
	pub fn set_value(&self, value: impl Into<T>) -> Option<T> {
		self.value.write().replace(value.into())
	}

	#[must_use]
	pub fn value(&self) -> Option<T> {
		self.value.write().take()
	}
}

#[cfg(test)]
mod tests {
	use alloc::{string::String, vec::Vec};

	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&OutPort<Vec<String>>>();
		is_normal::<OutPort<Vec<i32>>>();
	}
}
