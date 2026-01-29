// Copyright © 2026 Stephan Kunz
//! Module implementing various port collections.

use alloc::sync::Arc;

use crate::{
	any_port_value::AnyPortValue,
	bind::{
		BindCommons,
		port_value::{PortValueReadGuard, PortValueWriteGuard},
	},
	error::Error,
	port_variant::PortVariant,
};

pub mod port_array;
pub mod port_map;
pub mod port_vec;

/// Methods for something that is a collection of ports.
/// Each port is identified by its name, so the name has to be unique within a certain port collection.
pub trait PortCollection {
	/// Returns the port with 'name' from the collection.
	fn find(&self, name: &str) -> Option<&PortVariant>;

	/// Returns the mutable port with 'name' from the collection.
	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant>;
}

/// Methods for something that is a mutable collection of ports.
/// Each port is identified by its name, so the name has to be unique within a certain port collection.
pub trait PortCollectionMut {
	/// Adds the port under the given name to the collection;
	/// # Errors
	/// - [`Error::AlreadyInCollection`] if `name` is already in the collection.
	fn insert(&mut self, name: impl Into<Arc<str>>, port: PortVariant) -> Result<(), Error>;

	/// Removes the port with `name` from the collection and returns its value of type `T`.
	/// # Errors
	/// - [`Error::NotFound`] if `name` is not contained.
	/// - [`Error::DataType`] if 'name' has not the expected type `T`.
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
	/// - [`Error::NotFound`], if 'name' is not in port collection.
	fn sequence_number(&self, name: &str) -> Result<u32, Error>;
}

/// Access methods for port collections.
/// Each port is identified by its name, so the name has to be unique within a certain port collection.
pub trait PortCollectionAccessors: PortCollectionAccessorsCommon {
	/// Connects a port from this collection to the value of another port variant.
	/// Type of connection depends on types of both ports.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in collection.
	/// - [`Error::DataType`], if 'name' has not the same type of T as 'bound'.
	/// - [`Error::PortType`], if 'name' and 'bound' have incompatible port types.
	fn give_to_bound(&self, name: &str, bound: &mut impl BindCommons) -> Result<(), Error>;

	/// Connects a port from this collection to the value of another port variant.
	/// Type of connection depends on types of both ports.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in collection.
	/// - [`Error::DataType`], if 'name' has not the same type of T as 'variant'.
	/// - [`Error::PortType`], if 'name' and 'variant' have incompatible port types.
	fn give_to_variant(&self, name: &str, variant: &mut PortVariant) -> Result<(), Error>;

	/// Connects a port from this collection to a port from another collection.
	/// Type of connection depends on types of both ports.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in 'self' collection.
	/// - [`Error::OtherNotFound`], if 'name' is not in 'other' collection.
	/// - [`Error::DataType`], if 'name' has not the same type of T as 'other_name'.
	/// - [`Error::PortType`], if 'name' and 'other_name' have incompatible port types.
	fn give_to_collection(
		&self,
		name: &str,
		other_collection: &mut impl PortCollection,
		other_name: &str,
	) -> Result<(), Error>;

	/// Returns true if 'name' is in the port collection, otherwise false.
	fn contains_name(&self, name: &str) -> bool;

	/// Returns a result of `true` if a certain `key` of type `T` is available, otherwise a result of `false`.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in port collection.
	/// - [`Error::DataType`], if the 'name' exists, but has not the expected type `T`.
	fn contains<T: AnyPortValue>(&self, name: &str) -> Result<bool, Error>;

	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in port collection.
	/// - [`Error::DataType`], if 'name' has not the expected type of T.
	/// - [`Error::PortType`], if 'name' is not the expected port type.
	fn get<T>(&self, name: &str) -> Result<Option<T>, Error>
	where
		T: AnyPortValue + Clone;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in port collection.
	/// - [`Error::DataType`], if 'name' has not the expected type of T.
	/// - [`Error::PortType`], if 'name' is not the expected port type.
	fn read<T: AnyPortValue>(&self, name: &str) -> Result<PortValueReadGuard<T>, Error>;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if 'name' is locked.
	/// - [`Error::NotFound`], if 'name' is not in port collection.
	/// - [`Error::DataType`], if 'name' has not the expected type of T.
	/// - [`Error::PortType`], if 'name' is not the expected port type.
	fn try_read<T: AnyPortValue>(&self, name: &str) -> Result<PortValueReadGuard<T>, Error>;
}

pub trait PortCollectionAccessorsMut: PortCollectionAccessors {
	/// Connects a port from this collection to the value of another port variant.
	/// Type of connection depends on types of both ports.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in collection.
	/// - [`Error::DataType`], if 'name' has not the same type of T as 'bound'.
	/// - [`Error::PortType`], if 'name' and 'bound' have incompatible port types.
	fn use_from_bound(&mut self, name: &str, bound: &impl BindCommons) -> Result<(), Error>;

	/// Connects a port from this collection to the value of another port variant.
	/// Type of connection depends on types of both ports.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in collection.
	/// - [`Error::DataType`], if 'name' has not the same type of T as 'variant'.
	/// - [`Error::PortType`], if 'name' and 'variant' have incompatible port types.
	fn use_from_variant(&mut self, name: &str, variant: &PortVariant) -> Result<(), Error>;

	/// Connects a port from this collection to a port from another collection.
	/// Type of connection depends on types of both ports.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in 'self' collection.
	/// - [`Error::OtherNotFound`], if 'name' is not in 'other' collection.
	/// - [`Error::DataType`], if 'name' has not the same type of T as 'other_name'.
	/// - [`Error::PortType`], if 'name' and 'other_name' have incompatible port types.
	fn use_from_collection(
		&mut self,
		name: &str,
		other_collection: &impl PortCollection,
		other_name: &str,
	) -> Result<(), Error>;

	/// Sets a new value to the T and returns the old T.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in port collection.
	/// - [`Error::DataType`], if 'name' has not the expected type of T.
	/// - [`Error::PortType`], if 'name' is not the expected port type.
	fn replace<T: AnyPortValue>(&mut self, name: &str, value: T) -> Result<Option<T>, Error>;

	/// Sets a new value to the T.
	/// # Errors
	/// - [`Error::DataType`], if 'name' has not the expected type of T.
	/// - [`Error::PortType`], if 'name' is not the expected port type.
	fn set<T: AnyPortValue>(&mut self, name: &str, value: T) -> Result<(), Error>;

	/// Returns the T, removing it from the port.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in port collection.
	/// - [`Error::DataType`], if 'name' has not the expected type of T.
	/// - [`Error::PortType`], if 'name' is not the expected port type.
	fn take<T: AnyPortValue>(&mut self, name: &str) -> Result<Option<T>, Error>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if 'name' is not in port collection.
	/// - [`Error::DataType`], if 'name' has not the expected type of T.
	/// - [`Error::PortType`], if 'name' is not the expected port type.
	fn write<T: AnyPortValue>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>, Error>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if 'name' is locked.
	/// - [`Error::NotFound`], if 'name' is not in port collection.
	/// - [`Error::DataType`], if 'name' has not the expected type of T.
	/// - [`Error::PortType`], if 'name' is not the expected port type.
	fn try_write<T: AnyPortValue>(&mut self, name: &str) -> Result<PortValueWriteGuard<T>, Error>;
}

/// Trait for something that provides a collection of Ports.
pub trait PortCollectionProvider {
	/// Returns immutable access to the collections entries.
	fn provided_ports(&self) -> &impl PortCollectionAccessors;

	/// Returns mutable access to the collections entries.
	fn provided_ports_mut(&mut self) -> &mut impl PortCollectionAccessorsMut;

	/// Returns an immutable [`PortCollection`].
	fn port_collection(&self) -> &impl PortCollection;
}

/// Trait for something that provides a mutable collection of Ports.
pub trait PortCollectionProviderMut {
	/// Returns a mutable [`PortCollection`].
	fn port_collection_mut(&mut self) -> &mut impl PortCollectionMut;
}
