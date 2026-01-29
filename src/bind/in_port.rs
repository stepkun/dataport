// Copyright © 2026 Stephan Kunz
//! A bound input type port implementing [`BindIn`].

use alloc::{boxed::Box, sync::Arc};
use spin::RwLock;

use crate::{
	PortCollection,
	any_port_value::AnyPortValue,
	bind::{
		BindCommons, BindIn,
		port_value::{PortValue, PortValuePtr, PortValueReadGuard},
		sequence_number::SequenceNumber,
	},
	error::Error,
	port_variant::PortVariant,
};

/// @TODO:
#[derive(Debug, Clone)]
pub struct BoundInPort(PortValuePtr);

impl BoundInPort {
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
			Err(Error::DataType)
		}
	}

	pub(crate) fn into_inner<T: AnyPortValue>(self) -> Result<Option<T>, Error> {
		let any_value = &mut *self.0.write();
		let p = &mut any_value.0;
		let p_mut = p.as_mut();
		if let Some(t_ref) = p_mut.as_mut_any().downcast_mut::<PortValue<T>>() {
			any_value.1.increment();
			Ok(t_ref.take())
		} else {
			Err(Error::DataType)
		}
	}
}

impl BindCommons for BoundInPort {
	fn sequence_number(&self) -> u32 {
		self.0.read().1.value()
	}

	fn use_from_bound(&mut self, other: &impl BindCommons) -> Result<(), Error> {
		self.set_value(other.value())
	}

	fn use_from_variant(&mut self, other: &PortVariant) -> Result<(), Error> {
		match other {
			PortVariant::InBound(port) => self.use_from_bound(port),
			PortVariant::InOutBound(port) => self.use_from_bound(port),
			PortVariant::OutBound(port) => self.use_from_bound(port),
		}
	}

	fn use_from_collection(&mut self, name: &str, collection: &impl PortCollection) -> Result<(), Error> {
		if let Some(variant) = collection.find(name) {
			self.use_from_variant(variant)
		} else {
			Err(Error::OtherNotFound)
		}
	}

	fn value(&self) -> PortValuePtr {
		self.0.clone()
	}
}

impl<T: AnyPortValue> BindIn<T> for BoundInPort {
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
			Err(Error::DataType)
		}
	}

	fn read(&self) -> Result<PortValueReadGuard<T>, Error> {
		PortValueReadGuard::new(self.0.clone())
	}

	fn try_read(&self) -> Result<PortValueReadGuard<T>, Error> {
		PortValueReadGuard::try_new(self.0.clone())
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

	#[test]
	fn into_inner() {
		let port = BoundInPort::new::<i32>();
		assert_eq!(port.into_inner::<f64>(), Err(Error::DataType));
	}
}
