// Copyright © 2025 Stephan Kunz
//! Implementation of a port providing the trait [`InPort`].

use core::any::Any;

use crate::{
	ConstString, RwLock,
	error::{Error, Result},
	port_data::PortData,
	port_value::{PortValuePtr, PortValueReadGuard},
	traits::{InPort, PortCommons},
};

/// InPort
#[repr(transparent)]
pub struct InputPort<T>(RwLock<PortData<T>>);

impl<T> core::fmt::Debug for InputPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("InputPort").field(&self.0).finish()
	}
}

impl<T: 'static> PartialEq for InputPort<T> {
	/// Partial equality of an in port is, if both have the same name & value type
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

impl<T> PortCommons for InputPort<T> {
	fn name(&self) -> ConstString {
		self.0.read().name()
	}

	fn sequence_number(&self) -> u32 {
		self.0.read().sequence_number()
	}
}

impl<T> InPort<T> for InputPort<T> {
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
}

impl<T> InputPort<T> {
	#[must_use]
	pub fn new(name: impl Into<ConstString>) -> Self {
		Self(RwLock::new(PortData::new(name.into())))
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
		is_normal::<&InputPort<i32>>();
		is_normal::<InputPort<String>>();
	}
}
