// Copyright © 2026 Stephan Kunz
//! A bound input/output type port implementing [`BindIn`], [`BindOut`] and [`BindInOut`].

use alloc::{boxed::Box, sync::Arc};
use spin::RwLock;

use crate::{
	any_port_value::AnyPortValue,
	bind::{
		BindCommons, BindIn, BindInOut, BindOut,
		port_value::{PortValue, PortValuePtr, PortValueReadGuard, PortValueWriteGuard},
		sequence_number::SequenceNumber,
	},
	error::Error,
	port_variant::PortVariant,
};

/// @TODO:
#[derive(Debug)]
pub struct BoundInOutPort(PortValuePtr);

impl BoundInOutPort {
	pub fn new<T: AnyPortValue>() -> Self {
		Self(Arc::new(RwLock::new((
			Box::new(PortValue::<T>::empty()),
			SequenceNumber::default(),
		))))
	}

	pub fn with_value<T: AnyPortValue>(value: T) -> Self {
		let mut sq = SequenceNumber::default();
		sq.increment();
		Self(Arc::new(RwLock::new((Box::new(PortValue::new(value)), sq))))
	}

	pub(crate) fn is<T: AnyPortValue>(&self) -> bool {
		self.0
			.read()
			.0
			.as_ref()
			.as_any()
			.downcast_ref::<PortValue<T>>()
			.is_some()
	}

	pub(crate) fn set_value(&mut self, value: PortValuePtr) -> Result<(), Error> {
		let x = self.0.read().0.type_id();
		let y = value.read().0.type_id();
		if x == y {
			self.0 = value;
			Ok(())
		} else {
			Err(Error::WrongDataType)
		}
	}

	pub(crate) fn value(&self) -> PortValuePtr {
		self.0.clone()
	}
}

impl BindCommons for BoundInOutPort {
	fn bind_to(&mut self, other: &PortVariant) -> Result<(), Error> {
		match other {
			PortVariant::InBound(port) => self.set_value(port.value()),
			PortVariant::InOutBound(port) => self.set_value(port.value()),
			PortVariant::OutBound(port) => self.set_value(port.value()),
		}
	}

	fn sequence_number(&self) -> u32 {
		self.0.read().1.value()
	}
}

impl<T: AnyPortValue> BindIn<T> for BoundInOutPort {
	fn get(&self) -> Result<Option<T>, Error>
	where
		T: Clone,
	{
		let any_value = &*self.0.read();
		if let Some(t_ref) = any_value
			.0
			.as_ref()
			.as_any()
			.downcast_ref::<PortValue<T>>()
		{
			Ok(t_ref.get())
		} else {
			Err(Error::WrongDataType)
		}
	}

	fn read(&self) -> Result<PortValueReadGuard<T>, Error> {
		PortValueReadGuard::new(self.0.clone())
	}

	fn try_read(&self) -> Result<PortValueReadGuard<T>, Error> {
		PortValueReadGuard::try_new(self.0.clone())
	}
}

impl<T: AnyPortValue> BindInOut<T> for BoundInOutPort {
	fn replace(&mut self, value: T) -> Result<Option<T>, Error> {
		let any_value = &mut *self.0.write();
		let p = &mut any_value.0;
		let p_mut = p.as_mut();
		if let Some(t_ref) = p_mut.as_mut_any().downcast_mut::<PortValue<T>>() {
			any_value.1.increment();
			Ok(t_ref.replace(value))
		} else {
			Err(Error::WrongDataType)
		}
	}

	fn take(&mut self) -> Result<Option<T>, Error> {
		let any_value = &mut *self.0.write();
		let p = &mut any_value.0;
		let p_mut = p.as_mut();
		if let Some(t_ref) = p_mut.as_mut_any().downcast_mut::<PortValue<T>>() {
			any_value.1.increment();
			Ok(t_ref.take())
		} else {
			Err(Error::WrongDataType)
		}
	}
}

impl<T: AnyPortValue> BindOut<T> for BoundInOutPort {
	fn set(&mut self, value: T) -> Result<(), Error> {
		let any_value = &mut *self.0.write();
		let p = &mut any_value.0;
		let p_mut = p.as_mut();
		if let Some(t_ref) = p_mut.as_mut_any().downcast_mut::<PortValue<T>>() {
			t_ref.set(value);
			any_value.1.increment();
			Ok(())
		} else {
			Err(Error::WrongDataType)
		}
	}

	fn write(&mut self) -> Result<PortValueWriteGuard<T>, Error> {
		PortValueWriteGuard::new(self.0.clone())
	}

	fn try_write(&mut self) -> Result<PortValueWriteGuard<T>, Error> {
		PortValueWriteGuard::try_new(self.0.clone())
	}
}

impl Clone for BoundInOutPort {
	fn clone(&self) -> Self {
		BoundInOutPort(self.0.clone())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&BoundInOutPort>();
		is_normal::<BoundInOutPort>();
	}
}
