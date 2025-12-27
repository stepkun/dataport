// Copyright © 2025 Stephan Kunz
//! Implementation of a port providing [`OutPort`].

#![allow(unused)]

use core::{
	any::Any,
	num::{NonZero, NonZeroU32},
};

use alloc::sync::Arc;

use crate::{
	ConstString, RwLock,
	any_port::AnyPort,
	error::{Error, Result},
	port,
	port_value::{PortValue, PortValueReadGuard, PortValueWriteGuard},
	sequence_number::SequenceNumber,
	traits::{OutPort, PortBase},
};

/// OutputPort
pub struct OutputPort<T> {
	/// An identifying name of the port, which must be unique for a given item.
	name: ConstString,
	/// The current value of the port together with its change sequence.
	value: Arc<RwLock<PortValue<T>>>,
}

impl<T: 'static + Send + Sync> AnyPort for OutputPort<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

impl<T> core::fmt::Debug for OutputPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("OutputPort")
			.field("name", &self.name)
			//.field("value", &self.value)
			.finish_non_exhaustive()
	}
}

impl<T: 'static> PartialEq for OutputPort<T> {
	/// Partial equality of an out port is, if both have the same name & value type
	fn eq(&self, other: &Self) -> bool {
		if self.name == other.name {
			let v1 = self.value.read();
			let v2 = other.value.read();
			if let Some(value1) = v1.as_ref()
				&& let Some(value2) = v2.as_ref()
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

impl<T> PortBase for OutputPort<T> {
	fn name(&self) -> ConstString {
		self.name.clone()
	}
}

impl<T> OutPort<T> for OutputPort<T> {
	fn replace(&self, value: impl Into<T>) -> Option<T> {
		self.value.write().replace(value.into())
	}

	fn set(&self, value: impl Into<T>) {
		self.value.write().set(value.into())
	}

	fn write(&self) -> Result<PortValueWriteGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = self.value.read().is_some();
		if has_value {
			PortValueWriteGuard::new(self.name.clone(), self.value.clone())
		} else {
			Err(Error::NoValueSet { port: self.name.clone() })
		}
	}

	fn try_write(&self) -> Result<PortValueWriteGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = if let Some(guard) = self.value.try_read() {
			guard.is_some()
		} else {
			return Err(Error::IsLocked { port: self.name.clone() });
		};
		if has_value {
			PortValueWriteGuard::try_new(self.name.clone(), self.value.clone())
		} else {
			Err(Error::NoValueSet { port: self.name.clone() })
		}
	}
}

impl<T> OutputPort<T> {
	#[must_use]
	pub fn new(name: impl Into<ConstString>) -> Self {
		Self {
			name: name.into(),
			value: Arc::new(RwLock::new(PortValue::<T>::default())),
		}
	}

	#[must_use]
	pub fn with_value(name: impl Into<ConstString>, value: impl Into<T>) -> Self {
		let mut port_value = PortValue::<T>::default();
		port_value.set(value.into());
		Self {
			name: name.into(),
			value: Arc::new(RwLock::new(port_value)),
		}
	}

	/// Helper function to solve ambiguity.
	pub(crate) fn by_ref(&self) -> Result<PortValueReadGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = self.value.read().is_some();
		if has_value {
			PortValueReadGuard::new(self.name.clone(), self.value.clone())
		} else {
			Err(Error::NoValueSet { port: self.name.clone() })
		}
	}

	/// Helper function to solve ambiguity.
	pub(crate) fn try_by_ref(&self) -> Result<PortValueReadGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = if let Some(guard) = self.value.try_read() {
			guard.is_some()
		} else {
			return Err(Error::IsLocked { port: self.name.clone() });
		};
		if has_value {
			PortValueReadGuard::try_new(self.name.clone(), self.value.clone())
		} else {
			Err(Error::NoValueSet { port: self.name.clone() })
		}
	}

	#[must_use]
	pub(crate) fn by_copy(&self) -> Option<T>
	where
		T: Clone,
	{
		self.value.read().get()
	}

	#[must_use]
	pub(crate) fn by_value(&self) -> Option<T> {
		self.value.write().take()
	}

	#[must_use]
	pub(crate) fn sequence_id(&self) -> u32 {
		self.value.read().sequence_number()
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
		assert_eq!(o1.sequence_id(), 0);
		assert_eq!(o2.sequence_id(), 0);
		assert_eq!(o3.sequence_id(), 0);
		assert_eq!(o4.sequence_id(), 1);

		o1.set(42);
		o2.set(PI);
		o3.set(String::from("the answer"));
		assert_eq!(o1.sequence_id(), 1);
		assert_eq!(o2.sequence_id(), 1);
		assert_eq!(o3.sequence_id(), 1);

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

			assert_eq!(o1.sequence_id(), 1);
			assert_eq!(o2.sequence_id(), 1);
			assert_eq!(o3.sequence_id(), 1);
			assert_eq!(o4.sequence_id(), 1);
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
		assert_eq!(o1.sequence_id(), 1);
		assert_eq!(o2.sequence_id(), 1);
		assert_eq!(o3.sequence_id(), 1);
		assert_eq!(o4.sequence_id(), 1);

		assert_eq!(o1.by_value().unwrap(), 42);
		assert_eq!(o2.by_value().unwrap(), PI);
		assert_eq!(o3.by_value().unwrap(), String::from("the answer"));
		assert_eq!(o4.by_value().unwrap(), "hello world");

		assert_eq!(o1.sequence_id(), 2);
		assert_eq!(o2.sequence_id(), 2);
		assert_eq!(o3.sequence_id(), 2);
		assert_eq!(o4.sequence_id(), 2);

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
