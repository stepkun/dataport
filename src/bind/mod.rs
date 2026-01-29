// Copyright © 2026 Stephan Kunz
//! Module containing 'bind' type ports.

pub mod in_out_port;
pub mod in_port;
pub mod out_port;
pub mod port_value;
mod sequence_number;

use crate::{
	PortCollection,
	any_port_value::AnyPortValue,
	bind::port_value::{PortValuePtr, PortValueReadGuard, PortValueWriteGuard},
	error::Error,
	port_variant::PortVariant,
};

/// Trait for bind port types.
pub trait BindCommons {
	/// Binds this port to the port 'bound'.
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the same type 'T' as 'bound'.
	/// - [`Error::PortType`], if 'bound' is not a valid port type.
	fn use_from_bound(&mut self, bound: &impl BindCommons) -> Result<(), Error>;

	/// Binds this port to the port 'variant'.
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the same type 'T' as 'variant'.
	/// - [`Error::PortType`], if 'variant' is not a valid port type.
	fn use_from_variant(&mut self, variant: &PortVariant) -> Result<(), Error>;

	/// Binds this port to the port 'name' of 'collection'.
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the same type 'T' as 'name' in 'collection'.
	/// - [`Error::OtherNotFound`], if 'collection' does not contains port 'name'.
	/// - [`Error::PortType`], if 'variant' is not a valid port type.
	fn use_from_collection(&mut self, name: &str, collection: &impl PortCollection) -> Result<(), Error>;

	/// Returns the change sequence number of this port.
	/// A value of '0' indicates, that the port has never contained any data.
	fn sequence_number(&self) -> u32;

	/// Returns a pointer to the ports value.
	fn value(&self) -> PortValuePtr;
}

/// Trait for incoming bind port types.
pub trait BindIn<T: AnyPortValue>: BindCommons {
	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the expected type 'T'.
	fn get(&self) -> Result<Option<T>, Error>
	where
		T: Clone;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::DataType`], if port is not the expected port type & type of T.
	fn read(&self) -> Result<PortValueReadGuard<T>, Error>;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::DataType`], if port is not the expected port type & type of T.
	fn try_read(&self) -> Result<PortValueReadGuard<T>, Error>;
}

/// Trait for incoming and outgoing bind port types.
pub trait BindInOut<T: AnyPortValue>: BindIn<T> + BindOut<T> {
	/// Sets a new value to the T and returns the old T.
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the expected type 'T'.
	fn replace(&mut self, value: T) -> Result<Option<T>, Error>;

	/// Returns the T, removing it from the port.
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the expected type 'T'.
	fn take(&mut self) -> Result<Option<T>, Error>;
}

/// Trait for outgoing bind port types.
pub trait BindOut<T: AnyPortValue>: BindCommons {
	/// Sets a new value to the T.
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the expected type 'T'.
	fn set(&mut self, value: T) -> Result<(), Error>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::DataType`], if port is not the expected port type & type of T.
	fn write(&mut self) -> Result<PortValueWriteGuard<T>, Error>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::DataType`], if port is not the expected port type & type of T.
	fn try_write(&mut self) -> Result<PortValueWriteGuard<T>, Error>;
}
