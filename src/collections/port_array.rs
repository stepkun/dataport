// Copyright © 2026 Stephan Kunz
//! A fixed unsorted collection of ports.

use core::{cell::LazyCell, ops::Deref};

use crate::{
	ConstString,
	any_port_value::AnyPortValue,
	bind::{
		BindCommons,
		port_value::{PortValueReadGuard, PortValueWriteGuard},
	},
	collections::{PortCollection, PortCollectionAccessors, PortCollectionAccessorsCommon, PortCollectionAccessorsMut},
	error::Error,
	port_variant::PortVariant,
};

//static mut GLOBAL_BLACKBOARD: LazyCell<Databoard> = LazyCell::new(|| Databoard::default());
pub static mut EMPTY_PORT_ARRAY: LazyCell<PortArray<0>> = LazyCell::new(|| PortArray([]));

/// A fixed unsorted array of [`PortVariant`]s.
#[repr(transparent)]
pub struct PortArray<const S: usize>([(ConstString, PortVariant); S]);

impl<const S: usize> PortArray<S> {
	pub fn from(ports: [(ConstString, PortVariant); S]) -> Self {
		Self(ports)
	}
}

impl PortArray<0> {
	pub fn empty() -> Self {
		Self::from([])
	}
}

impl<const S: usize> core::fmt::Debug for PortArray<S> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("PortArray").field(&self.0).finish()
	}
}

impl<const S: usize> Deref for PortArray<S> {
	type Target = [(ConstString, PortVariant)];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<const S: usize> PortCollection for PortArray<S> {
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

impl<const S: usize> PortCollectionAccessorsCommon for PortArray<S> {
	fn sequence_number(&self, name: &str) -> Result<u32, Error> {
		if let Some(port) = self.find(name) {
			Ok(port.sequence_number())
		} else {
			Err(Error::NotFound)
		}
	}
}

impl<const S: usize> PortCollectionAccessors for PortArray<S> {
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

impl<const S: usize> PortCollectionAccessorsMut for PortArray<S> {
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
		is_normal::<&PortArray<2>>();
		is_normal::<PortArray<4>>();
	}
}
