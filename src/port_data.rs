// Copyright © 2025 Stephan Kunz
//! Generic implementation of a port.

#![allow(unused)]

use core::any::Any;

use alloc::sync::Arc;

use crate::{
	ConstString, Error, RwLock,
	error::Result,
	port_value::{PortValue, PortValueReadGuard, PortValueWriteGuard},
	traits::{AnyPort, InPort, OutPort, PortCommons},
};

/// PortData.
pub(crate) struct PortData<T> {
	/// An identifying name of the port, which must be unique for a given item.
	name: ConstString,
	/// The current value of the port together with its change sequence.
	value: Arc<RwLock<PortValue<T>>>,
}

impl<T: 'static + Send + Sync> AnyPort for PortData<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

impl<T> core::fmt::Debug for PortData<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("InputOutputPort")
			.field("name", &self.name())
			//.field("input", &self.input)
			//.field("output", &self.output)
			.finish_non_exhaustive()
	}
}

impl<T: 'static> PartialEq for PortData<T> {
	/// Partial equality of a port is, if name and value type are the same
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

impl<T> PortCommons for PortData<T> {
	fn name(&self) -> ConstString {
		self.name.clone()
	}

	fn sequence_number(&self) -> u32 {
		self.value.read().sequence_number()
	}
}

impl<T> InPort<T> for PortData<T> {
	fn get(&self) -> Option<T>
	where
		T: Clone,
	{
		self.value.read().get()
	}

	fn read(&self) -> Result<PortValueReadGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = self.value.read().is_some();
		if has_value {
			PortValueReadGuard::new(self.name.clone(), self.value.clone())
		} else {
			Err(Error::ValueNotSet { port: self.name.clone() })
		}
	}

	fn try_read(&self) -> Result<PortValueReadGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = if let Some(guard) = self.value.try_read() {
			guard.is_some()
		} else {
			return Err(Error::IsLocked { port: self.name.clone() });
		};
		if has_value {
			PortValueReadGuard::try_new(self.name.clone(), self.value.clone())
		} else {
			Err(Error::ValueNotSet { port: self.name.clone() })
		}
	}
}

impl<T> OutPort<T> for PortData<T> {
	fn replace(&self, value: impl Into<T>) -> Option<T> {
		self.value.write().replace(value.into())
	}

	fn set(&self, value: impl Into<T>) {
		self.value.write().set(value.into())
	}

	fn take(&self) -> Option<T> {
		self.value.write().take()
	}

	fn write(&self) -> Result<PortValueWriteGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = self.value.read().is_some();
		if has_value {
			PortValueWriteGuard::new(self.name.clone(), self.value.clone())
		} else {
			Err(Error::ValueNotSet { port: self.name.clone() })
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
			Err(Error::ValueNotSet { port: self.name.clone() })
		}
	}
}

impl<T> PortData<T> {
	#[must_use]
	pub(crate) fn new(name: impl Into<ConstString>) -> Self {
		Self {
			name: name.into(),
			value: Arc::new(RwLock::new(PortValue::default())),
		}
	}

	#[must_use]
	pub(crate) fn with_value(name: impl Into<ConstString>, value: impl Into<T>) -> Self {
		Self {
			name: name.into(),
			value: Arc::new(RwLock::new(PortValue::new(value.into()))),
		}
	}

	pub(crate) fn value(&self) -> Arc<RwLock<PortValue<T>>> {
		self.value.clone()
	}

	pub(crate) fn set_value(&mut self, value: Arc<RwLock<PortValue<T>>>) {
		self.value = value
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
		is_normal::<&PortData<f32>>();
		is_normal::<PortData<String>>();
	}
}
