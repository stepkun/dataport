// Copyright © 2026 Stephan Kunz
//! An extendable sorted collection of ports.

use core::ops::{Deref, DerefMut};

use alloc::collections::btree_map::{BTreeMap, Entry};

use crate::{
	ConstString,
	any_port_value::AnyPortValue,
	bind::{
		BindCommons,
		port_value::{PortValueReadGuard, PortValueWriteGuard},
	},
	collections::{
		PortCollection, PortCollectionAccessors, PortCollectionAccessorsCommon, PortCollectionAccessorsMut, PortCollectionMut,
	},
	error::Error,
	port_variant::PortVariant,
};

/// An extendable sorted map of [`PortVariant`]s.
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct PortMap(BTreeMap<ConstString, PortVariant>);

impl PortMap {
	pub fn from<const N: usize>(array: [(ConstString, PortVariant); N]) -> Self {
		Self(BTreeMap::from(array))
	}
}

impl Deref for PortMap {
	type Target = BTreeMap<ConstString, PortVariant>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for PortMap {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl PortCollection for PortMap {
	fn find(&self, name: &str) -> Option<&PortVariant> {
		self.0.get(name)
	}

	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant> {
		self.0.get_mut(name)
	}
}

impl PortCollectionMut for PortMap {
	fn insert(&mut self, name: impl Into<ConstString>, port: PortVariant) -> Result<(), Error> {
		let name = name.into();
		match self.0.entry(name.clone()) {
			Entry::Vacant(vacant_entry) => {
				vacant_entry.insert(port);
				Ok(())
			}
			Entry::Occupied(_) => Err(Error::AlreadyInCollection),
		}
	}

	fn remove<T: AnyPortValue>(&mut self, name: impl Into<ConstString>) -> Result<Option<T>, Error> {
		let name = name.into();
		match self.0.entry(name.clone()) {
			Entry::Vacant(_) => Err(Error::NotFound),
			Entry::Occupied(occupied_entry) => {
				let value = occupied_entry.get();
				if value.is::<T>() {
					occupied_entry.remove().into_inner::<T>()
				} else {
					Err(Error::DataType)
				}
			}
		}
	}
}

impl PortCollectionAccessorsCommon for PortMap {
	fn sequence_number(&self, name: &str) -> Result<u32, Error> {
		if let Some(port) = self.find(name) {
			Ok(port.sequence_number())
		} else {
			Err(Error::NotFound)
		}
	}
}

impl PortCollectionAccessors for PortMap {
	fn give_to_bound(&self, name: &str, bound: &mut impl BindCommons) -> Result<(), Error> {
		self.find(name)
			.map_or(Err(Error::NotFound), |port| bound.use_from_variant(port))
	}

	fn give_to_variant(&self, name: &str, variant: &mut PortVariant) -> Result<(), Error> {
		self.find(name)
			.map_or(Err(Error::NotFound), |port| variant.use_from_variant(port))
	}

	fn give_to_collection(
		&self,
		name: &str,
		other_collection: &mut impl PortCollection,
		other_name: &str,
	) -> Result<(), Error> {
		self.find(name)
			.map_or(Err(Error::NotFound), |port| {
				other_collection
					.find_mut(other_name)
					.map_or(Err(Error::OtherNotFound), |variant| variant.use_from_variant(port))
			})
	}

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

impl PortCollectionAccessorsMut for PortMap {
	fn use_from_bound(&mut self, name: &str, bound: &impl BindCommons) -> Result<(), Error> {
		self.find_mut(name)
			.map_or(Err(Error::NotFound), |port| port.use_from_bound(bound))
	}

	fn use_from_variant(&mut self, name: &str, variant: &PortVariant) -> Result<(), Error> {
		self.find_mut(name)
			.map_or(Err(Error::NotFound), |port| port.use_from_variant(variant))
	}

	fn use_from_collection(
		&mut self,
		name: &str,
		other_collection: &impl PortCollection,
		other_name: &str,
	) -> Result<(), Error> {
		self.find_mut(name)
			.map_or(Err(Error::NotFound), |port| {
				port.use_from_collection(other_collection, other_name)
			})
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
		is_normal::<&PortMap>();
		is_normal::<PortMap>();
	}
}
