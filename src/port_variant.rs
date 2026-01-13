// Copyright © 2026 Stephan Kunz
//! Port variants.

use crate::{
	any_port_value::AnyPortValue,
	bind::{
		BindCommons, BindIn, BindInOut, BindOut,
		in_out_port::BoundInOutPort,
		in_port::BoundInPort,
		out_port::BoundOutPort,
		port_value::{PortValueReadGuard, PortValueWriteGuard},
	},
	error::{Error, Result},
};

/// Implemented set of port variants.
/// - InBound: bound to some other ports value, only readable
/// - InOutBound: bound to some other ports value, read- & writeable
/// - OutBound: bound to some other ports value, only writeable
#[allow(clippy::enum_variant_names)] // the flow variants to be expected
#[derive(Debug, Clone)]
pub enum PortVariant {
	InBound(BoundInPort),
	InOutBound(BoundInOutPort),
	OutBound(BoundOutPort),
}

impl PortVariant {
	pub fn create_inbound<T: AnyPortValue>(value: T) -> Self {
		Self::InBound(BoundInPort::with_value(value))
	}

	pub fn create_inoutbound<T: AnyPortValue>(value: T) -> Self {
		Self::InOutBound(BoundInOutPort::with_value(value))
	}

	pub fn create_outbound<T: AnyPortValue>(value: T) -> Self {
		Self::OutBound(BoundOutPort::with_value(value))
	}

	pub fn connect_to(&mut self, other: &PortVariant) -> Result<()> {
		match self {
			Self::InBound(port) => port.bind_to(other),
			Self::InOutBound(port) => port.bind_to(other),
			Self::OutBound(port) => port.bind_to(other),
		}
	}

	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
	pub fn get<T: AnyPortValue + Clone>(&self) -> Result<Option<T>> {
		match self {
			Self::InBound(port) => port.get(),
			Self::InOutBound(port) => port.get(),
			Self::OutBound(_) => Err(Error::WrongPortType),
		}
	}

	pub fn is<T: AnyPortValue>(&self) -> bool {
		match self {
			Self::InBound(port) => port.is::<T>(),
			Self::InOutBound(port) => port.is::<T>(),
			Self::OutBound(port) => port.is::<T>(),
		}
	}

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	pub fn read<T: AnyPortValue>(&self) -> Result<PortValueReadGuard<T>> {
		match self {
			Self::InBound(port) => port.read(),
			Self::InOutBound(port) => port.read(),
			Self::OutBound(_) => Err(Error::WrongPortType),
		}
	}

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`](crate::error::Error), if port is locked.
	/// - [`Error::WrongDataType`](crate::error::Error), if port is not the expected port type & type of T.
	pub fn try_read<T: AnyPortValue>(&self) -> Result<PortValueReadGuard<T>> {
		match self {
			Self::InBound(port) => port.try_read(),
			Self::InOutBound(port) => port.try_read(),
			Self::OutBound(_) => Err(Error::WrongPortType),
		}
	}

	/// Sets a new value to the T and returns the old T.
	pub fn replace<T: AnyPortValue>(&mut self, value: T) -> Result<Option<T>> {
		match self {
			Self::InOutBound(port) => port.replace(value),
			Self::InBound(_) | Self::OutBound(_) => Err(Error::WrongPortType),
		}
	}

	/// Returns the change sequence number,
	/// a number which
	/// - starts at `0`,
	/// - can only be incremeted by 1 and
	/// - wraps around to `1` when exceeding its limits.
	pub fn sequence_number(&self) -> u32 {
		match self {
			Self::InBound(port) => port.sequence_number(),
			Self::InOutBound(port) => port.sequence_number(),
			Self::OutBound(port) => port.sequence_number(),
		}
	}

	/// Returns the T, removing it from the port.
	pub fn take<T: AnyPortValue>(&mut self) -> Result<Option<T>> {
		match self {
			Self::InOutBound(port) => port.take(),
			Self::InBound(_) | Self::OutBound(_) => Err(Error::WrongPortType),
		}
	}

	/// Sets a new value to the T.
	pub fn set<T: AnyPortValue>(&mut self, value: T) -> Result<()> {
		match self {
			Self::OutBound(port) => port.set(value),
			Self::InOutBound(port) => port.set(value),
			Self::InBound(_) => Err(Error::WrongPortType),
		}
	}

	pub fn write<T: AnyPortValue>(&mut self) -> Result<PortValueWriteGuard<T>> {
		match self {
			Self::OutBound(port) => port.write(),
			Self::InOutBound(port) => port.write(),
			Self::InBound(_) => Err(Error::WrongPortType),
		}
	}

	pub fn try_write<T: AnyPortValue>(&mut self) -> Result<PortValueWriteGuard<T>> {
		match self {
			Self::OutBound(port) => port.try_write(),
			Self::InOutBound(port) => port.try_write(),
			Self::InBound(_) => Err(Error::WrongPortType),
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
		is_normal::<&PortVariant>();
		is_normal::<PortVariant>();
	}
}
