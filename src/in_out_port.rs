// Copyright © 2025 Stephan Kunz
//! Implementation of a port providing both traits: [`InBound`], [`OutBound`] and [`InOutBound`].

use core::any::Any;

use crate::{
	ConstString, RwLock,
	error::{Error, Result},
	out_port::OutBoundPort,
	port_data::PortData,
	port_value::{PortValuePtr, PortValueReadGuard, PortValueWriteGuard},
	traits::{InBound, InOutBound, OutBound, PortCommons},
};

/// InOutBoundPort
#[repr(transparent)]
pub struct InOutBoundPort<T>(RwLock<PortData<T>>);

impl<T> Clone for InOutBoundPort<T> {
	fn clone(&self) -> Self {
		Self(RwLock::new((*self.0.read()).clone()))
	}
}

impl<T> core::fmt::Debug for InOutBoundPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("InOutBoundPort")
			.field(&self.0)
			.finish()
	}
}

impl<T: 'static> PartialEq for InOutBoundPort<T> {
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

impl<T> PortCommons for InOutBoundPort<T> {
	fn name(&self) -> ConstString {
		self.0.read().name()
	}

	fn sequence_number(&self) -> u32 {
		self.0.read().sequence_number()
	}
}

impl<T> InBound<T> for InOutBoundPort<T> {
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
			Err(Error::NoValueSet {
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
			Err(Error::NoValueSet {
				port: self.0.read().name(),
			})
		}
	}
}

impl<T> InOutBound<T> for InOutBoundPort<T> {
	fn replace(&self, value: impl Into<T>) -> Option<T> {
		self.0
			.read()
			.value()
			.write()
			.replace(value.into())
	}

	fn take(&self) -> Option<T> {
		self.0.read().value().write().take()
	}
}

impl<T> OutBound<T> for InOutBoundPort<T> {
	fn set(&self, value: impl Into<T>) {
		self.0.read().value().write().set(value.into())
	}

	fn write(&self) -> Result<PortValueWriteGuard<T>> {
		// Test for value is separate to not pass a locked value into the guard.
		let has_value = self.0.read().value().read().is_some();
		if has_value {
			PortValueWriteGuard::new(self.0.read().name(), self.0.read().value())
		} else {
			Err(Error::NoValueSet {
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
			Err(Error::NoValueSet {
				port: self.0.read().name(),
			})
		}
	}
}

impl<T> InOutBoundPort<T> {
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

	pub fn bind_to_out_port(&mut self, port: &OutBoundPort<T>) -> Result<()> {
		if self.value().read().is_some() {
			return Err(Error::AlreadyBound { port: self.name() });
		}
		self.set_value(port.value());
		Ok(())
	}

	pub fn bind_to_in_out_port(&mut self, port: &InOutBoundPort<T>) -> Result<()> {
		if self.value().read().is_some() {
			return Err(Error::AlreadyBound { port: self.name() });
		}
		self.set_value(port.value());
		Ok(())
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
		is_normal::<&InOutBoundPort<f32>>();
		is_normal::<InOutBoundPort<String>>();
	}
}
