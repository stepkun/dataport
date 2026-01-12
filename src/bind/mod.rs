// Copyright © 2026 Stephan Kunz
//! Module containing 'bind' type ports.

pub mod any_port_value;
pub mod in_out_port;
pub mod in_port;
pub mod out_port;
pub mod port_value;
mod sequence_number;

use crate::{
	bind::{
		any_port_value::AnyPortValueType,
		port_value::{PortValueReadGuard, PortValueWriteGuard},
	},
	error::Result,
	port_variant::PortVariant,
};

/// Trait for bind port types.
pub trait BindCommons {
	/// Binds a port to another port variant.
	fn bind_to(&mut self, other: &PortVariant) -> Result<()>;

	/// Returns change sequence number.
	fn sequence_number(&self) -> u32;
}

/// Trait for incoming bind port types.
pub trait BindIn<T: AnyPortValueType>: BindCommons {
	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
	fn get(&self) -> Result<Option<T>>
	where
		T: Clone;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn read(&self) -> Result<PortValueReadGuard<T>>;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`](crate::error::Error), if port is locked.
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn try_read(&self) -> Result<PortValueReadGuard<T>>;
}

/// Trait for incoming and outgoing bind port types.
pub trait BindInOut<T: AnyPortValueType>: BindIn<T> + BindOut<T> {
	/// Sets a new value to the T and returns the old T.
	fn replace(&mut self, value: T) -> Result<Option<T>>;

	/// Returns the T, removing it from the port.
	fn take(&mut self) -> Result<Option<T>>;
}

/// Trait for outgoing bind port types.
pub trait BindOut<T: AnyPortValueType>: BindCommons {
	/// Sets a new value to the T.
	fn set(&mut self, value: T) -> Result<()>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn write(&mut self) -> Result<PortValueWriteGuard<T>>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`](crate::error::Error), if port is locked.
	/// - [`Error::NotFound`](crate::error::Error), if port is not in port list.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	fn try_write(&mut self) -> Result<PortValueWriteGuard<T>>;
}
