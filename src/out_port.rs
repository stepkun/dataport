// Copyright © 2025 Stephan Kunz
//! Implementation of a port providing [`OutPort`].

#![allow(unused)]

use core::any::Any;

use alloc::sync::Arc;

use crate::{
	ConstString, RwLock,
	error::{Error, Result},
	port_data::PortData,
	port_value::{PortValue, PortValueReadGuard, PortValueWriteGuard},
	traits::{AnyPort, OutPort, PortCommons},
};

/// OutputPort
pub struct OutputPort<T> {
	data: RwLock<PortData<T>>,
}

impl<T> core::fmt::Debug for OutputPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("OutputPort")
			.field("data", &self.data)
			.finish()
	}
}

impl<T: 'static> PartialEq for OutputPort<T> {
	/// Partial equality of an out port is, if both have the same name & value type
	fn eq(&self, other: &Self) -> bool {
		if self.data.read().name() == other.data.read().name() {
			let v1 = self.data.read().value();
			let v2 = other.data.read().value();
			// check type of v1 against type of v2
			if v1.type_id() == v2.type_id() {
				return true;
			}
		}
		false
	}
}

impl<T> PortCommons for OutputPort<T> {
	fn name(&self) -> ConstString {
		self.data.read().name()
	}

	fn sequence_number(&self) -> u32 {
		self.data.read().sequence_number()
	}
}

impl<T> OutPort<T> for OutputPort<T> {
	fn replace(&self, value: impl Into<T>) -> Option<T> {
		self.data
			.read()
			.value()
			.write()
			.replace(value.into())
	}

	fn set(&self, value: impl Into<T>) {
		self.data.read().value().write().set(value.into())
	}

	fn take(&self) -> Option<T> {
		self.data.read().value().write().take()
	}

	fn write(&self) -> Result<PortValueWriteGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = self.data.read().value().read().is_some();
		if has_value {
			PortValueWriteGuard::new(self.data.read().name(), self.data.read().value())
		} else {
			Err(Error::ValueNotSet {
				port: self.data.read().name(),
			})
		}
	}

	fn try_write(&self) -> Result<PortValueWriteGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = if let Some(guard) = self.data.read().value().try_read() {
			guard.is_some()
		} else {
			return Err(Error::IsLocked {
				port: self.data.read().name(),
			});
		};
		if has_value {
			PortValueWriteGuard::try_new(self.data.read().name(), self.data.read().value())
		} else {
			Err(Error::ValueNotSet {
				port: self.data.read().name(),
			})
		}
	}
}

impl<T> OutputPort<T> {
	#[must_use]
	pub fn new(name: impl Into<ConstString>) -> Self {
		Self {
			data: RwLock::new(PortData::new(name.into())),
		}
	}

	#[must_use]
	pub(crate) fn with_value(name: impl Into<ConstString>, value: impl Into<T>) -> Self {
		Self {
			data: RwLock::new(PortData::with_value(name.into(), value.into())),
		}
	}

	/// Helper function to solve ambiguity.
	pub(crate) fn by_ref(&self) -> Result<PortValueReadGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = self.data.read().value().read().is_some();
		if has_value {
			PortValueReadGuard::new(self.data.read().name(), self.data.read().value())
		} else {
			Err(Error::ValueNotSet {
				port: self.data.read().name(),
			})
		}
	}

	/// Helper function to solve ambiguity.
	pub(crate) fn try_by_ref(&self) -> Result<PortValueReadGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = if let Some(guard) = self.data.read().value().try_read() {
			guard.is_some()
		} else {
			return Err(Error::IsLocked {
				port: self.data.read().name(),
			});
		};
		if has_value {
			PortValueReadGuard::try_new(self.data.read().name(), self.data.read().value())
		} else {
			Err(Error::ValueNotSet {
				port: self.data.read().name(),
			})
		}
	}

	#[must_use]
	pub(crate) fn by_copy(&self) -> Option<T>
	where
		T: Clone,
	{
		self.data.read().value().read().get()
	}

	#[must_use]
	pub(crate) fn by_value(&self) -> Option<T> {
		self.data.read().value().write().take()
	}

	pub(crate) fn value(&self) -> Arc<RwLock<PortValue<T>>> {
		self.data.read().value()
	}

	pub(crate) fn set_value(&self, value: Arc<RwLock<PortValue<T>>>) {
		self.data.write().set_value(value);
	}
}

#[cfg(test)]
mod tests {
	use core::{f64::consts::PI, ops::Deref};

	use alloc::{string::String, vec::Vec};

	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	const CONST_NAME: &str = "p2";
	static STATIC_NAME: &str = "p3";

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&OutputPort<Vec<String>>>();
		is_normal::<OutputPort<Vec<i32>>>();
	}

	#[test]
	fn non_public_accessors() {
		let o1 = OutputPort::<i32>::new("p1");
		let o2 = OutputPort::<f64>::new(CONST_NAME);
		let o3 = OutputPort::<String>::new(STATIC_NAME);
		let o4 = OutputPort::<&str>::with_value("p4", "hello world");
		assert_eq!(o1.sequence_number(), 0);
		assert_eq!(o2.sequence_number(), 0);
		assert_eq!(o3.sequence_number(), 0);
		assert_eq!(o4.sequence_number(), 1);

		o1.set(42);
		o2.set(PI);
		o3.set(String::from("the answer"));
		assert_eq!(o1.sequence_number(), 1);
		assert_eq!(o2.sequence_number(), 1);
		assert_eq!(o3.sequence_number(), 1);
		assert_eq!(o4.sequence_number(), 1);

		// separate block to release locks
		{
			let o1_guard = o1.by_ref().unwrap();
			let o2_guard = o2.by_ref().unwrap();
			let o3_guard = o3.by_ref().unwrap();
			let o4_guard = o4.by_ref().unwrap();

			assert_eq!(o1_guard.deref(), &42);
			assert_eq!(o2_guard.deref(), &PI);
			assert_eq!(o3_guard.deref(), &String::from("the answer"));
			assert_eq!(*o4_guard, "hello world");

			assert_eq!(o1.try_by_ref().unwrap().deref(), &42);
			assert_eq!(*o1.try_by_ref().unwrap(), 42);
			assert_eq!(o2.try_by_ref().unwrap().deref(), &PI);
			assert_eq!(*o2.try_by_ref().unwrap(), PI);
			assert_eq!(o3.try_by_ref().unwrap().deref(), &String::from("the answer"));
			assert_eq!(*o3.try_by_ref().unwrap(), String::from("the answer"));
			assert_eq!(*o4.try_by_ref().unwrap(), "hello world");

			assert!(o1.try_write().is_err());
			assert!(o2.try_write().is_err());
			assert!(o3.try_write().is_err());
			assert!(o4.try_write().is_err());

			assert_eq!(o1.sequence_number(), 1);
			assert_eq!(o2.sequence_number(), 1);
			assert_eq!(o3.sequence_number(), 1);
			assert_eq!(o4.sequence_number(), 1);
		}

		// separate block to release locks
		{
			let o1_guard = o1.write().unwrap();
			let o2_guard = o2.write().unwrap();
			let o3_guard = o3.write().unwrap();
			let o4_guard = o4.write().unwrap();

			assert_eq!(o1_guard.deref(), &42);
			assert_eq!(o2_guard.deref(), &PI);
			assert_eq!(o3_guard.deref(), &String::from("the answer"));
			assert_eq!(*o4_guard, "hello world");

			assert!(o1.try_by_ref().is_err());
			assert!(o2.try_by_ref().is_err());
			assert!(o3.try_by_ref().is_err());
			assert!(o4.try_by_ref().is_err());
		}
		assert_eq!(o1.sequence_number(), 1);
		assert_eq!(o2.sequence_number(), 1);
		assert_eq!(o3.sequence_number(), 1);
		assert_eq!(o4.sequence_number(), 1);

		assert_eq!(o1.by_value().unwrap(), 42);
		assert_eq!(o2.by_value().unwrap(), PI);
		assert_eq!(o3.by_value().unwrap(), String::from("the answer"));
		assert_eq!(o4.by_value().unwrap(), "hello world");

		assert_eq!(o1.sequence_number(), 2);
		assert_eq!(o2.sequence_number(), 2);
		assert_eq!(o3.sequence_number(), 2);
		assert_eq!(o4.sequence_number(), 2);

		assert!(o1.by_ref().is_err());
		assert!(o1.try_by_ref().is_err());
		assert!(o2.by_ref().is_err());
		assert!(o2.try_by_ref().is_err());
		assert!(o3.by_ref().is_err());
		assert!(o3.try_by_ref().is_err());
		assert!(o4.by_ref().is_err());
		assert!(o4.try_by_ref().is_err());
	}
}
