// Copyright © 2025 Stephan Kunz
//! Implementation of a port providing both [`InPort`] and [`OutPort`].

#![allow(unused)]

use core::any::Any;

use alloc::sync::Arc;

use crate::{
	ConstString, Error, RwLock,
	error::Result,
	port_data::PortData,
	port_value::{PortValue, PortValueReadGuard, PortValueWriteGuard},
	traits::{AnyPort, InPort, OutPort, PortCommons},
};

/// InputOutputPort
pub struct InputOutputPort<T> {
	data: RwLock<PortData<T>>,
}

impl<T: 'static + Send + Sync> AnyPort for InputOutputPort<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

impl<T> core::fmt::Debug for InputOutputPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("InputOutputPort")
			.field("data", &self.data)
			.finish()
	}
}

impl<T: 'static> PartialEq for InputOutputPort<T> {
	/// Partial equality of an in/out port is, if input and output parts are partial equal
	fn eq(&self, other: &Self) -> bool {
		todo!();
		false
	}
}

impl<T> PortCommons for InputOutputPort<T> {
	fn name(&self) -> ConstString {
		self.data.read().name()
	}

	fn sequence_number(&self) -> u32 {
		self.data.read().sequence_number()
	}
}

impl<T> InPort<T> for InputOutputPort<T> {
	fn get(&self) -> Option<T>
	where
		T: Clone,
	{
		self.data.read().get()
	}

	fn read(&self) -> Result<PortValueReadGuard<T>> {
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

	fn try_read(&self) -> Result<PortValueReadGuard<T>> {
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
}

impl<T> OutPort<T> for InputOutputPort<T> {
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

impl<T> InputOutputPort<T> {
	#[must_use]
	pub fn new(name: impl Into<ConstString>) -> Self {
		Self {
			data: RwLock::new(PortData::new(name.into())),
		}
	}

	#[must_use]
	pub fn with_value(name: impl Into<ConstString>, value: impl Into<T>) -> Self {
		Self {
			data: RwLock::new(PortData::with_value(name.into(), value.into())),
		}
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
