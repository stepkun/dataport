// Copyright © 2026 Stephan Kunz
//! An extendable sorted collection of ports.

use core::ops::{Deref, DerefMut};

use alloc::collections::btree_map::{BTreeMap, Entry};

use crate::{
	ConstString,
	any_port_value::AnyPortValue,
	collections::{PortCollection, PortCollectionMut},
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
