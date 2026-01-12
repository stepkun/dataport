// Copyright © 2026 Stephan Kunz
//! An extendable unsorted collection of ports.

use alloc::vec::Vec;

use crate::{
	ConstString, DynamicPortProvider,
	bind::any_port_value::AnyPortValueType,
	collections::{PortAccessors, PortProvider},
	error::{Error, Result},
	port_variant::PortVariant,
};

/// An extendable unsorted list of [`PortVariant`]s.
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct PortList(Vec<(ConstString, PortVariant)>);

impl PortList {
	pub fn new() -> Self {
		Self(Vec::new())
	}
}

impl PortProvider for PortList {
	fn contains_key(&self, name: &str) -> bool {
		self.0.iter().any(|port| &*port.0 == name)
	}

	fn find(&self, name: &str) -> Option<&PortVariant> {
		self.0
			.iter()
			.find(|port| &*port.0 == name)
			.map(|v| &v.1 as _)
	}

	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant> {
		self.0
			.iter_mut()
			.find(|port| &*port.0 == name)
			.map(|v| &mut v.1 as _)
	}
}

impl DynamicPortProvider for PortList {
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
		// @TODO: improve performance by doing a better search for name
		if self.find(&name).is_some() {
			Err(Error::AlreadyInCollection { name })
		} else {
			self.0.push((name, port));
			Ok(())
		}
	}

	fn remove(&mut self, name: impl Into<ConstString>) -> Result<PortVariant> {
		let name = name.into();
		let index = self.0.iter().position(|r| r.0 == name);
		if let Some(index) = index {
			let res = self.0.remove(index);
			Ok(res.1)
		} else {
			Err(Error::NotFound { name })
		}
	}
}

impl PortAccessors for PortList {}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&PortList>();
		is_normal::<PortList>();
	}
}
