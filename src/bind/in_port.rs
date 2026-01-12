// Copyright © 2026 Stephan Kunz
//! A bound input type port implementing [`BindIn`].

use alloc::{boxed::Box, sync::Arc};
use spin::RwLock;

use crate::{
	bind::{
		BindCommons, BindIn,
		any_port_value::AnyPortValueType,
		port_value::{PortValue, PortValuePtr, PortValueReadGuard},
		sequence_number::SequenceNumber,
	},
	error::{Error, Result},
	port_variant::PortVariant,
};

/// @TODO:
#[derive(Debug)]
pub struct BoundInPort(PortValuePtr);

impl BoundInPort {
	pub fn new<T: AnyPortValueType>() -> Self {
		Self(Arc::new(RwLock::new((
			Box::new(PortValue::<T>::empty()),
			SequenceNumber::default(),
		))))
	}

	pub fn with_value<T: AnyPortValueType>(value: T) -> Self {
		let mut sq = SequenceNumber::default();
		sq.increment();
		Self(Arc::new(RwLock::new((Box::new(PortValue::new(value)), sq))))
	}

	pub(crate) fn is<T: AnyPortValueType>(&self) -> bool {
		self.0
			.read()
			.0
			.as_ref()
			.as_any()
			.downcast_ref::<PortValue<T>>()
			.is_some()
	}

	pub(crate) fn set_value(&mut self, value: PortValuePtr) -> Result<()> {
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

impl BindCommons for BoundInPort {
	fn bind_to(&mut self, other: &PortVariant) -> Result<()> {
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

impl<T: AnyPortValueType> BindIn<T> for BoundInPort {
	fn get(&self) -> Result<Option<T>>
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

	fn read(&self) -> Result<PortValueReadGuard<T>> {
		PortValueReadGuard::new(self.0.clone())
	}

	fn try_read(&self) -> Result<PortValueReadGuard<T>> {
		PortValueReadGuard::try_new(self.0.clone())
	}
}

impl Clone for BoundInPort {
	fn clone(&self) -> Self {
		BoundInPort(self.0.clone())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&BoundInPort>();
		is_normal::<BoundInPort>();
	}
}
