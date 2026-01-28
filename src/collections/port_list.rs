// Copyright © 2026 Stephan Kunz
//! An extendable unsorted collection of ports.

use core::ops::{Deref, DerefMut};

use alloc::vec::Vec;

use crate::{
	ConstString,
	any_port_value::AnyPortValue,
	bind::port_value::{PortValueReadGuard, PortValueWriteGuard},
	collections::{
		PortCollection, PortCollectionAccessors, PortCollectionAccessorsCommon, PortCollectionAccessorsMut, PortProvider,
	},
	error::Error,
	port_variant::PortVariant,
};

/// An extendable unsorted list of [`PortVariant`]s.
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct PortList(Vec<(ConstString, PortVariant)>);

impl PortList {
	pub fn from<const N: usize>(array: [(ConstString, PortVariant); N]) -> Self {
		Self(Vec::from(array))
	}
}

impl Deref for PortList {
	type Target = Vec<(ConstString, PortVariant)>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for PortList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl PortCollection for PortList {
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

	fn connect_with(&mut self, name: &str, other_collection: &impl PortCollection, other_name: &str) -> Result<(), Error> {
		if let Some(other) = other_collection.find(other_name) {
			self.use_value_from(name, other)
		} else {
			Err(Error::OtherNotFound)
		}
	}
}

impl PortProvider for PortList {
	fn insert(&mut self, name: impl Into<ConstString>, port: PortVariant) -> Result<(), Error> {
		let name = name.into();
		if self.find(&name).is_some() {
			Err(Error::AlreadyInCollection)
		} else {
			self.0.push((name, port));
			Ok(())
		}
	}

	fn remove<T: AnyPortValue>(&mut self, name: impl Into<ConstString>) -> Result<Option<T>, Error> {
		let name = name.into();
		match self.contains::<T>(&name) {
			Ok(found) => {
				if found {
					// remove should not fail due to `contains` test above
					let index = self
						.0
						.iter()
						.position(|r| r.0 == name)
						.expect("unreachable");
					self.0.remove(index).1.into_inner::<T>()
				} else {
					Err(Error::NotFound)
				}
			}
			Err(err) => Err(err),
		}
	}
}

impl PortCollectionAccessorsCommon for PortList {
	fn sequence_number(&self, name: &str) -> Result<u32, Error> {
		if let Some(port) = self.find(name) {
			Ok(port.sequence_number())
		} else {
			Err(Error::NotFound)
		}
	}
}

impl PortCollectionAccessors for PortList {
	fn contains_name(&self, name: &str) -> bool {
		self.find(name).is_some()
	}

	fn contains<T: AnyPortValue>(&self, name: &str) -> Result<bool, Error> {
		if let Some(p) = self.find(name) {
			if p.is::<T>() { Ok(true) } else { Err(Error::DataType) }
		} else {
			Ok(false)
		}
	}

	fn get<T>(&self, name: &str) -> Result<Option<T>, Error>
	where
		T: AnyPortValue + Clone,
	{
		if let Some(port) = self.find(name) {
			port.get()
		} else {
			Err(Error::NotFound)
		}
	}

	fn read<T: AnyPortValue>(&self, name: &str) -> Result<PortValueReadGuard<T>, Error> {
		if let Some(port) = self.find(name) {
			port.read()
		} else {
			Err(Error::NotFound)
		}
	}

	fn try_read<T: AnyPortValue>(&self, name: &str) -> Result<PortValueReadGuard<T>, Error> {
		if let Some(port) = self.find(name) {
			port.try_read()
		} else {
			Err(Error::NotFound)
		}
	}
}

impl PortCollectionAccessorsMut for PortList {
	fn use_value_from(&mut self, name: &str, port: &PortVariant) -> Result<(), Error> {
		if let Some(self_port) = self.find_mut(name) {
			self_port.use_value_from(port)
		} else {
			Err(Error::NotFound)
		}
	}

	fn replace<T: AnyPortValue>(&mut self, name: &str, value: T) -> Result<Option<T>, Error> {
		if let Some(port) = self.find_mut(name) {
			port.replace(value)
		} else {
			Err(Error::NotFound)
		}
	}

	fn set<T: AnyPortValue>(&mut self, name: &str, value: T) -> Result<(), Error> {
		if let Some(port) = self.find_mut(name) {
			port.set(value)
		} else {
			Err(Error::NotFound)
		}
	}

	fn take<T: AnyPortValue>(&mut self, name: &str) -> Result<Option<T>, Error> {
		if let Some(port) = self.find_mut(name) {
			port.take()
		} else {
			Err(Error::NotFound)
		}
	}

	fn write<T: AnyPortValue>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>, Error> {
		if let Some(port) = self.find_mut(name) {
			port.write()
		} else {
			Err(Error::NotFound)
		}
	}

	fn try_write<T: AnyPortValue>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>, Error> {
		if let Some(port) = self.find_mut(name) {
			port.try_write()
		} else {
			Err(Error::NotFound)
		}
	}
}

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
