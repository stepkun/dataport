// Copyright © 2026 Stephan Kunz
//! Module implementing port collections.

use crate::{
	ConstString,
	any_port_value::AnyPortValue,
	bind::port_value::{PortValueReadGuard, PortValueWriteGuard},
	error::Error,
	port_variant::PortVariant,
};

pub mod port_array;
pub mod port_list;
pub mod port_map;

pub trait PortProvider: PortCollection + PortCollectionAccessors {}

/// Blanket implementation
impl<S: PortCollection + PortCollectionAccessors> PortProvider for S {}

pub trait PortProviderMut: PortCollection + DynamicPortCollection + PortCollectionAccessors {}

/// Blanket implementation
impl<S: PortCollection + DynamicPortCollection + PortCollectionAccessors> PortProviderMut for S {}

/// Methods for something that provides a collection of ports.
/// Each port is identified by its name, so the name has to be unique within a certain port collection.
pub trait PortCollection {
	/// Returns the port from the collection.
	fn find(&self, name: &str) -> Option<&PortVariant>;

	/// Returns the mutable port from the collection.
	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant>;
}

/// Methods for something that is able to provide ports as dynamic collection.
/// Each port is identified by its name, so the name has to be unique within a certain port collection.
pub trait DynamicPortCollection {
	/// Returns the value of type `T` stored under `name` and removes the port from collection.
	/// # Errors
	/// - [`Error::NotFound`] if `name` is not contained.
	/// - [`Error::WrongDataType`] if the port has not the expected type `T`.
	fn delete<T: AnyPortValue>(&mut self, name: &str) -> Result<Option<T>, Error>;

	/// Adds the port under the given name to the collection;
	/// # Errors
	/// - [`Error::AlreadyInCollection`] if `name` is already contained.
	fn insert(&mut self, name: impl Into<ConstString>, port: PortVariant) -> Result<(), Error>;

	/// Removes the port with the given name from the collection;
	/// # Errors
	/// - [`Error::NotFound`] if `name` is not contained.
	fn remove(&mut self, name: impl Into<ConstString>) -> Result<PortVariant, Error>;
}

/// Access methods for port collections.
/// Each port is identified by its name, so the name has to be unique within a certain port collection.
pub trait PortCollectionAccessors {
	/// Connects a port from this collection to a port from another collection.
	/// Type of connection depends on types of both ports.
	/// # Errors
	/// - [`Error::NotFound`], if one of the ports is not in the given port list.
	/// - [`Error::WrongDataType`], if a port has not the expected type of T.
	/// - [`Error::WrongPortType`], if a port is not the expected port type.
	fn connect_to(&mut self, name: &str, other_collection: &impl PortCollection, other_name: &str) -> Result<(), Error>;

	/// Returns true if the name is in the port collection.
	fn contains_name(&self, name: &str) -> bool;

	/// Returns a result of `true` if a certain `key` of type `T` is available, otherwise a result of `false`.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongDataType`] if the port exists, but has not the expected type `T`.
	fn contains<T: AnyPortValue>(&self, name: &str) -> Result<bool, Error>;

	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongDataType`], if port has not the expected type of T.
	/// - [`Error::WrongPortType`], if port is not the expected port type.
	fn get<T>(&self, name: &str) -> Result<Option<T>, Error>
	where
		T: AnyPortValue + Clone;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongDataType`], if port has not the expected type of T.
	/// - [`Error::WrongPortType`], if port is not the expected port type.
	fn read<T: AnyPortValue>(&self, name: &str) -> Result<PortValueReadGuard<T>, Error>;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongDataType`], if port has not the expected type of T.
	/// - [`Error::WrongPortType`], if port is not the expected port type.
	fn try_read<T: AnyPortValue>(&self, name: &str) -> Result<PortValueReadGuard<T>, Error>;

	/// Sets a new value to the T and returns the old T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongDataType`], if port has not the expected type of T.
	/// - [`Error::WrongPortType`], if port is not the expected port type.
	fn replace<T: AnyPortValue>(&mut self, name: &str, value: T) -> Result<Option<T>, Error>;

	/// Returns the change sequence number,
	/// a number which
	/// - starts at `0`,
	/// - can only be incremeted by 1 and
	/// - wraps around to `1` when exceeding its limits.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	fn sequence_number(&self, name: &str) -> Result<u32, Error>;

	/// Sets a new value to the T.
	/// # Errors
	/// - [`Error::WrongDataType`], if port has not the expected type of T.
	/// - [`Error::WrongPortType`], if port is not the expected port type.
	fn set<T: AnyPortValue>(&mut self, name: &str, value: T) -> Result<(), Error>;

	/// Returns the T, removing it from the port.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongDataType`], if port has not the expected type of T.
	/// - [`Error::WrongPortType`], if port is not the expected port type.
	fn take<T: AnyPortValue>(&mut self, name: &str) -> Result<Option<T>, Error>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongDataType`], if port has not the expected type of T.
	/// - [`Error::WrongPortType`], if port is not the expected port type.
	fn write<T: AnyPortValue>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>, Error>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongDataType`], if port has not the expected type of T.
	/// - [`Error::WrongPortType`], if port is not the expected port type.
	fn try_write<T: AnyPortValue>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>, Error>;
}

/// Blanket implementation for [`PortCollection`]s.
impl<S: PortCollection> PortCollectionAccessors for S {
	fn connect_to(&mut self, name: &str, other_collection: &impl PortCollection, other_name: &str) -> Result<(), Error> {
		if let Some(port) = self.find_mut(name) {
			if let Some(other) = other_collection.find(other_name) {
				port.connect_to(other)
			} else {
				Err(Error::NotFound { name: other_name.into() })
			}
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	fn contains_name(&self, name: &str) -> bool {
		self.find(name).is_some()
	}

	fn contains<T: AnyPortValue>(&self, name: &str) -> Result<bool, Error> {
		if let Some(p) = self.find(name) {
			if p.is::<T>() { Ok(true) } else { Err(Error::WrongDataType) }
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
			Err(Error::NotFound { name: name.into() })
		}
	}

	fn read<T: AnyPortValue>(&self, name: &str) -> Result<PortValueReadGuard<T>, Error> {
		if let Some(port) = self.find(name) {
			port.read()
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	fn try_read<T: AnyPortValue>(&self, name: &str) -> Result<PortValueReadGuard<T>, Error> {
		if let Some(port) = self.find(name) {
			port.try_read()
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	fn replace<T: AnyPortValue>(&mut self, name: &str, value: T) -> Result<Option<T>, Error> {
		if let Some(port) = self.find_mut(name) {
			port.replace(value)
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	fn sequence_number(&self, name: &str) -> Result<u32, Error> {
		if let Some(port) = self.find(name) {
			Ok(port.sequence_number())
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	fn set<T: AnyPortValue>(&mut self, name: &str, value: T) -> Result<(), Error> {
		if let Some(port) = self.find_mut(name) {
			port.set(value)
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	fn take<T: AnyPortValue>(&mut self, name: &str) -> Result<Option<T>, Error> {
		if let Some(port) = self.find_mut(name) {
			port.take()
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	fn write<T: AnyPortValue>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>, Error> {
		if let Some(port) = self.find_mut(name) {
			port.write()
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}

	fn try_write<T: AnyPortValue>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>, Error> {
		if let Some(port) = self.find_mut(name) {
			port.try_write()
		} else {
			Err(Error::NotFound { name: name.into() })
		}
	}
}
