// Copyright © 2026 Stephan Kunz
//! An extendable sorted collection of ports.

use alloc::collections::btree_map::BTreeMap;

use crate::{
	ConstString,
	bind::{any_port_value::AnyPortValueType, port_value::PortValue},
	collections::{DynamicPortProvider, PortAccessors, PortProvider},
	error::{Error, Result},
	port_variant::PortVariant,
};

/// An extendable sorted map of [`PortVariant`]s.
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct PortMap(BTreeMap<ConstString, PortVariant>);

impl PortMap {
	pub fn new() -> Self {
		Self(BTreeMap::new())
	}
}

impl PortProvider for PortMap {
	fn contains_key(&self, name: &str) -> bool {
		self.0.contains_key(name)
	}

	fn contains<T: AnyPortValueType>(&self, name: &str) -> Result<bool> {
		match self.0.get(name) {
			Some(port) => match port {
				PortVariant::InBound(port) => {
					let value = port.value();
					let guard = value.read();
					let x = &*guard;
					if x.0
						.as_ref()
						.as_any()
						.downcast_ref::<PortValue<T>>()
						.is_some()
					{
						Ok(true)
					} else {
						Err(Error::WrongDataType)
					}
				}
				PortVariant::InOutBound(port) => {
					let value = port.value();
					let guard = value.read();
					let x = &*guard;
					if x.0
						.as_ref()
						.as_any()
						.downcast_ref::<PortValue<T>>()
						.is_some()
					{
						Ok(true)
					} else {
						Err(Error::WrongDataType)
					}
				}
				PortVariant::OutBound(port) => {
					let value = port.value();
					let guard = value.read();
					let x = &*guard;
					if x.0
						.as_ref()
						.as_any()
						.downcast_ref::<PortValue<T>>()
						.is_some()
					{
						Ok(true)
					} else {
						Err(Error::WrongDataType)
					}
				}
			},
			None => Ok(false),
		}
	}

	fn find(&self, name: &str) -> Option<&PortVariant> {
		self.0.get(name)
	}

	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant> {
		self.0.get_mut(name)
	}
}

impl DynamicPortProvider for PortMap {
	fn delete<T: AnyPortValueType>(&mut self, name: &str) -> Result<Option<T>> {
		match self.contains::<T>(name) {
			Ok(found) => {
				if found {
					let value = self.take::<T>(name);
					self.remove(name)?; // this should not fail due to contains check above
					value
				} else {
					Err(Error::NotFound { name: name.into() })
				}
			}
			Err(err) => Err(err),
		}
	}

	fn insert(&mut self, name: impl Into<ConstString>, port: PortVariant) -> Result<()> {
		let name = name.into();
		if self.find(&name).is_some() {
			Err(Error::AlreadyInCollection { name })
		} else {
			self.0.insert(name, port);
			Ok(())
		}
	}

	fn remove(&mut self, name: impl Into<ConstString>) -> Result<PortVariant> {
		let name = name.into();
		if let Some(port) = self.0.remove(&name) {
			Ok(port)
		} else {
			Err(Error::NotFound { name })
		}
	}
}

impl PortAccessors for PortMap {}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&PortMap>();
		is_normal::<PortMap>();
	}
}
