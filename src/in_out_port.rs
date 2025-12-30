// Copyright © 2025 Stephan Kunz
//! Implementation of a port providing both traits: [`InPort`] and [`OutPort`].

use core::any::Any;

use crate::{
	ConstString, RwLock,
	error::{Error, Result},
	port_data::PortData,
	port_value::{PortValuePtr, PortValueReadGuard, PortValueWriteGuard},
	traits::{InOutPort, InPort, OutPort, PortCommons},
};

/// InputOutputPort
#[repr(transparent)]
pub struct InputOutputPort<T>(RwLock<PortData<T>>);

impl<T> core::fmt::Debug for InputOutputPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("InputOutputPort")
			.field(&self.0)
			.finish()
	}
}

impl<T: 'static> PartialEq for InputOutputPort<T> {
	/// Partial equality of an in/out port is, if both have the same name & value type
	fn eq(&self, other: &Self) -> bool {
		if self.0.read().name() == other.0.read().name() {
			let v1 = self.0.read().value();
			let v2 = other.0.read().value();
			// check type of v1 against type of v2
			if v1.type_id() == v2.type_id() {
				return true;
			}
		}
		false
	}
}

impl<T> PortCommons for InputOutputPort<T> {
	fn name(&self) -> ConstString {
		self.0.read().name()
	}

	fn sequence_number(&self) -> u32 {
		self.0.read().sequence_number()
	}
}

impl<T> InPort<T> for InputOutputPort<T> {
	fn get(&self) -> Option<T>
	where
		T: Clone,
	{
		self.0.read().get()
	}

	fn read(&self) -> Result<PortValueReadGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = self.0.read().value().read().is_some();
		if has_value {
			PortValueReadGuard::new(self.0.read().name(), self.0.read().value())
		} else {
			Err(Error::ValueNotSet {
				port: self.0.read().name(),
			})
		}
	}

	fn try_read(&self) -> Result<PortValueReadGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = if let Some(guard) = self.0.read().value().try_read() {
			guard.is_some()
		} else {
			return Err(Error::IsLocked {
				port: self.0.read().name(),
			});
		};
		if has_value {
			PortValueReadGuard::try_new(self.0.read().name(), self.0.read().value())
		} else {
			Err(Error::ValueNotSet {
				port: self.0.read().name(),
			})
		}
	}

	fn take(&self) -> Option<T> {
		self.0.read().value().write().take()
	}
}

impl<T> InOutPort<T> for InputOutputPort<T> {
	fn replace(&self, value: impl Into<T>) -> Option<T> {
		self.0
			.read()
			.value()
			.write()
			.replace(value.into())
	}
}

impl<T> OutPort<T> for InputOutputPort<T> {
	fn set(&self, value: impl Into<T>) {
		self.0.read().value().write().set(value.into())
	}

	fn write(&self) -> Result<PortValueWriteGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = self.0.read().value().read().is_some();
		if has_value {
			PortValueWriteGuard::new(self.0.read().name(), self.0.read().value())
		} else {
			Err(Error::ValueNotSet {
				port: self.0.read().name(),
			})
		}
	}

	fn try_write(&self) -> Result<PortValueWriteGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = if let Some(guard) = self.0.read().value().try_read() {
			guard.is_some()
		} else {
			return Err(Error::IsLocked {
				port: self.0.read().name(),
			});
		};
		if has_value {
			PortValueWriteGuard::try_new(self.0.read().name(), self.0.read().value())
		} else {
			Err(Error::ValueNotSet {
				port: self.0.read().name(),
			})
		}
	}
}

impl<T> InputOutputPort<T> {
	#[must_use]
	pub fn new(name: impl Into<ConstString>) -> Self {
		Self(RwLock::new(PortData::new(name.into())))
	}

	#[must_use]
	pub fn with_value(name: impl Into<ConstString>, value: impl Into<T>) -> Self {
		Self(RwLock::new(PortData::with_value(name.into(), value.into())))
	}

	pub(crate) fn value(&self) -> PortValuePtr<T> {
		self.0.read().value()
	}

	pub(crate) fn set_value(&self, value: PortValuePtr<T>) {
		self.0.write().set_value(value);
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
		is_normal::<&InputOutputPort<f32>>();
		is_normal::<InputOutputPort<String>>();
	}
}
