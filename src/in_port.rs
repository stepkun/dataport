// Copyright © 2025 Stephan Kunz
//! Implementation of a port providing [`InPort`].

#![allow(unused)]

use core::any::Any;

use alloc::sync::Arc;

use crate::{
	ConstString, Error, RwLock,
	error::Result,
	port_data::PortData,
	port_value::{PortValue, PortValueReadGuard},
	traits::{AnyPort, InPort, PortBase},
};

/// InPort
pub struct InputPort<T> {
	data: RwLock<PortData<T>>,
}

impl<T: 'static + Send + Sync> AnyPort for InputPort<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

impl<T> core::fmt::Debug for InputPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("InputPort")
			.field("data", &self.data)
			.finish()
	}
}

impl<T: 'static> PartialEq for InputPort<T> {
	/// Partial equality of an in port is, if both have the same name & value type
	fn eq(&self, other: &Self) -> bool {
		todo!()
		//		if self.data.read().name() == other.data.read().name() {
		//			let v1 = self.value.read();
		//			let v2 = other.value.read();
		//			if let Some(value1) = v1.as_ref()
		//				&& let Some(value2) = v2.as_ref()
		//			{
		//				// check type of value1 against type of value2
		//				if value1.type_id() == value2.type_id() {
		//					return true;
		//				}
		//			} else if v1.is_none() && v2.is_none() {
		//				return true;
		//			}
		//		}
		//		false
	}
}

impl<T> PortBase for InputPort<T> {
	fn name(&self) -> ConstString {
		self.data.read().name()
	}

	fn sequence_number(&self) -> u32 {
		self.data.read().sequence_number()
	}
}

impl<T> InPort<T> for InputPort<T> {
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

impl<T> InputPort<T> {
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
		is_normal::<&InputPort<i32>>();
		is_normal::<InputPort<String>>();
	}
}
