// Copyright © 2026 Stephan Kunz
//! A bound output type port implementing [`BindOut`].

use alloc::{boxed::Box, sync::Arc};
use spin::RwLock;

use crate::{
	any_port_value::AnyPortValue,
	bind::{
		BindCommons, BindOut,
		port_value::{PortValue, PortValuePtr, PortValueWriteGuard},
		sequence_number::SequenceNumber,
	},
	error::Error,
	port_variant::PortVariant,
};

/// @TODO:
#[derive(Debug)]
pub struct BoundOutPort(PortValuePtr);

impl BoundOutPort {
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

impl BindCommons for BoundOutPort {
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

impl<T: AnyPortValue> BindOut<T> for BoundOutPort {
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

impl Clone for BoundOutPort {
	fn clone(&self) -> Self {
		BoundOutPort(self.0.clone())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&BoundOutPort>();
		is_normal::<BoundOutPort>();
	}
}
