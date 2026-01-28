// Copyright © 2026 Stephan Kunz
//! Module implementing port collections.

use alloc::sync::Arc;

use crate::{
	any_port_value::AnyPortValue,
	bind::port_value::{PortValueReadGuard, PortValueWriteGuard},
	error::Error,
	port_variant::PortVariant,
};

pub mod port_array;
pub mod port_list;
pub mod port_map;

/// Methods for something that provides a collection of ports.
/// Each port is identified by its name, so the name has to be unique within a certain port collection.
pub trait PortCollection {
	/// Returns the port from the collection.
	fn find(&self, name: &str) -> Option<&PortVariant>;

	/// Returns the mutable port from the collection.
	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant>;

	/// Connects a port from this collection to a port from another collection.
	/// Type of connection depends on types of both ports.
	/// # Errors
	/// - [`Error::NotFound`], if the port is not in 'self' collection.
	/// - [`Error::OtherNotFound`], if the port is not in 'other' collection.
	/// - [`Error::DataType`], if a port has not the expected type of T.
	/// - [`Error::PortType`], if a port is not the expected port type.
	fn connect_with(&mut self, name: &str, other_collection: &impl PortCollection, other_name: &str) -> Result<(), Error>;
}

/// Methods for something that is able to provide ports as a dynamic collection.
/// Each port is identified by its name, so the name has to be unique within a certain port collection.
pub trait PortProvider {
	/// Adds the port under the given name to the collection;
	/// # Errors
	/// - [`Error::AlreadyInCollection`] if `name` is already contained.
	fn insert(&mut self, name: impl Into<Arc<str>>, port: PortVariant) -> Result<(), Error>;

	/// Removes the port with `name` from the collection and returns its value of type `T`.
	/// # Errors
	/// - [`Error::NotFound`] if `name` is not contained.
	/// - [`Error::DataType`] if the port has not the expected type `T`.
	fn remove<T: AnyPortValue>(&mut self, name: impl Into<Arc<str>>) -> Result<Option<T>, Error>;
}

/// Common access methods for port collections.
/// Each port is identified by its name, so the name has to be unique within a certain port collection.
pub trait PortCollectionAccessorsCommon {
	/// Returns the change sequence number,
	/// a number which
	/// - starts at `0`,
	/// - can only be incremeted by 1 and
	/// - wraps around to `1` when exceeding its limits.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port collection.
	fn sequence_number(&self, name: &str) -> Result<u32, Error>;
}

/// Access methods for port collections.
/// Each port is identified by its name, so the name has to be unique within a certain port collection.
pub trait PortCollectionAccessors: PortCollectionAccessorsCommon {
	/// Returns true if the name is in the port collection.
	fn contains_name(&self, name: &str) -> bool;

	/// Returns a result of `true` if a certain `key` of type `T` is available, otherwise a result of `false`.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port collection.
	/// - [`Error::DataType`] if the port exists, but has not the expected type `T`.
	fn contains<T: AnyPortValue>(&self, name: &str) -> Result<bool, Error>;

	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port collection.
	/// - [`Error::DataType`], if port has not the expected type of T.
	/// - [`Error::PortType`], if port is not the expected port type.
	fn get<T>(&self, name: &str) -> Result<Option<T>, Error>
	where
		T: AnyPortValue + Clone;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port collection.
	/// - [`Error::DataType`], if port has not the expected type of T.
	/// - [`Error::PortType`], if port is not the expected port type.
	fn read<T: AnyPortValue>(&self, name: &str) -> Result<PortValueReadGuard<T>, Error>;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port collection.
	/// - [`Error::DataType`], if port has not the expected type of T.
	/// - [`Error::PortType`], if port is not the expected port type.
	fn try_read<T: AnyPortValue>(&self, name: &str) -> Result<PortValueReadGuard<T>, Error>;
}

pub trait PortCollectionAccessorsMut: PortCollectionAccessors {
	/// Connects a port from this collection to the value of another port variant.
	/// Type of connection depends on types of both ports.
	/// # Errors
	/// - [`Error::NotFound`], if the port is not in port collection.
	/// - [`Error::DataType`], if a port has not the expected type of T.
	/// - [`Error::PortType`], if a port is not the expected port type.
	fn use_value_from(&mut self, name: &str, port: &PortVariant) -> Result<(), Error>;

	/// Sets a new value to the T and returns the old T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port collection.
	/// - [`Error::DataType`], if port has not the expected type of T.
	/// - [`Error::PortType`], if port is not the expected port type.
	fn replace<T: AnyPortValue>(&mut self, name: &str, value: T) -> Result<Option<T>, Error>;

	/// Sets a new value to the T.
	/// # Errors
	/// - [`Error::DataType`], if port has not the expected type of T.
	/// - [`Error::PortType`], if port is not the expected port type.
	fn set<T: AnyPortValue>(&mut self, name: &str, value: T) -> Result<(), Error>;

	/// Returns the T, removing it from the port.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port collection.
	/// - [`Error::DataType`], if port has not the expected type of T.
	/// - [`Error::PortType`], if port is not the expected port type.
	fn take<T: AnyPortValue>(&mut self, name: &str) -> Result<Option<T>, Error>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port collection.
	/// - [`Error::DataType`], if port has not the expected type of T.
	/// - [`Error::PortType`], if port is not the expected port type.
	fn write<T: AnyPortValue>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>, Error>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port collection.
	/// - [`Error::DataType`], if port has not the expected type of T.
	/// - [`Error::PortType`], if port is not the expected port type.
	fn try_write<T: AnyPortValue>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>, Error>;
}
