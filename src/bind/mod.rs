// Copyright © 2026 Stephan Kunz
//! Module containing 'bind' type ports.

pub mod in_out_port;
pub mod in_port;
pub mod out_port;
pub mod port_value;
mod sequence_number;

use crate::{
	any_port_value::AnyPortValue,
	bind::port_value::{PortValueReadGuard, PortValueWriteGuard},
	error::Error,
	port_variant::PortVariant,
};

/// Trait for bind port types.
pub trait BindCommons {
	/// Binds a port to another port variant.
	fn bind_to(&mut self, other: &PortVariant) -> Result<(), Error>;

	/// Returns change sequence number.
	fn sequence_number(&self) -> u32;
}

/// Trait for incoming bind port types.
pub trait BindIn<T: AnyPortValue>: BindCommons {
	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
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
	fn replace(&mut self, value: T) -> Result<Option<T>, Error>;

	/// Returns the T, removing it from the port.
	fn take(&mut self) -> Result<Option<T>, Error>;
}

/// Trait for outgoing bind port types.
pub trait BindOut<T: AnyPortValue>: BindCommons {
	/// Sets a new value to the T.
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
