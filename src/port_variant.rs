// Copyright © 2026 Stephan Kunz
//! Port variants.

use crate::{
	PortCollection,
	any_port_value::AnyPortValue,
	bind::{
		BindCommons, BindIn, BindInOut, BindOut,
		in_out_port::BoundInOutPort,
		in_port::BoundInPort,
		out_port::BoundOutPort,
		port_value::{PortValueReadGuard, PortValueWriteGuard},
	},
	error::Error,
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
	/// Returns a [`PortVariant::InBound`] with the 'value' set.
	pub fn create_inbound<T: AnyPortValue>(value: T) -> Self {
		Self::InBound(BoundInPort::with_value(value))
	}

	/// Returns a [`PortVariant::InOutBound`] with the 'value' set.
	pub fn create_inoutbound<T: AnyPortValue>(value: T) -> Self {
		Self::InOutBound(BoundInOutPort::with_value(value))
	}

	/// Returns a [`PortVariant::OutBound`] with the 'value' set.
	pub fn create_outbound<T: AnyPortValue>(value: T) -> Self {
		Self::OutBound(BoundOutPort::with_value(value))
	}

	/// Binds this port to the port 'bound'.
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the same type 'T' as 'bound'.
	/// - [`Error::PortType`], if 'bound' is not a valid port type.
	pub fn use_from_bound(&mut self, bound: &impl BindCommons) -> Result<(), Error> {
		match self {
			Self::InBound(port) => port.use_from_bound(bound),
			Self::InOutBound(port) => port.use_from_bound(bound),
			Self::OutBound(port) => port.use_from_bound(bound),
		}
	}

	/// Binds this port to the port 'variant'.
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the same type 'T' as 'variant'.
	/// - [`Error::PortType`], if 'variant' is not a valid port type.
	pub fn use_from_variant(&mut self, other: &PortVariant) -> Result<(), Error> {
		match other {
			Self::InBound(bound) => self.use_from_bound(bound),
			Self::InOutBound(bound) => self.use_from_bound(bound),
			Self::OutBound(bound) => self.use_from_bound(bound),
		}
	}

	/// Binds this port to the port 'name' of 'collection'.
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the same type 'T' as 'name' in 'collection'.
	/// - [`Error::OtherNotFound`], if 'collection' does not contains port 'name'.
	/// - [`Error::PortType`], if 'variant' is not a valid port type.
	pub fn use_from_collection(&mut self, collection: &impl PortCollection, name: &str) -> Result<(), Error> {
		if let Some(variant) = collection.find(name) {
			self.use_from_variant(variant)
		} else {
			Err(Error::OtherNotFound)
		}
	}

	/// Returns a clone/copy of the T.
	/// Therefore T must implement [`Clone`].
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the expected type 'T'.
	/// - [`Error::PortType`], if method is not available on 'self'.
	pub fn get<T: AnyPortValue + Clone>(&self) -> Result<Option<T>, Error> {
		match self {
			Self::InBound(port) => port.get(),
			Self::InOutBound(port) => port.get(),
			Self::OutBound(_) => Err(Error::PortType),
		}
	}

	/// Checks type 'T' of port.
	/// # Errors
	/// - [`Error::DataType`], if 'self' is not the expected type 'T'.
	pub fn is<T: AnyPortValue>(&self) -> bool {
		match self {
			Self::InBound(port) => port.is::<T>(),
			Self::InOutBound(port) => port.is::<T>(),
			Self::OutBound(port) => port.is::<T>(),
		}
	}

	/// Returns an immutable guard to the ports value 'T'.
	/// # Errors
	/// - [`Error::DataType`](crate::error::Error), if port is not the expected port type & type of T.
	pub fn read<T: AnyPortValue>(&self) -> Result<PortValueReadGuard<T>, Error> {
		match self {
			Self::InBound(port) => port.read(),
			Self::InOutBound(port) => port.read(),
			Self::OutBound(_) => Err(Error::PortType),
		}
	}

	/// Returns an immutable guard to the ports value 'T'.
	/// # Errors
	/// - [`Error::IsLocked`](crate::error::Error), if port is locked.
	/// - [`Error::DataType`](crate::error::Error), if port is not the expected port type & type of T.
	pub fn try_read<T: AnyPortValue>(&self) -> Result<PortValueReadGuard<T>, Error> {
		match self {
			Self::InBound(port) => port.try_read(),
			Self::InOutBound(port) => port.try_read(),
			Self::OutBound(_) => Err(Error::PortType),
		}
	}

	/// Sets a new value to the T and returns the old T.
	pub fn replace<T: AnyPortValue>(&mut self, value: T) -> Result<Option<T>, Error> {
		match self {
			Self::InOutBound(port) => port.replace(value),
			Self::InBound(_) | Self::OutBound(_) => Err(Error::PortType),
		}
	}

	/// Returns the change sequence number, a number which
	/// - starts at `0` for a port which never contained a value,
	/// - incremets by 1 whenever the ports value changes
	/// - wraps around to `1` when exceeding [`u32::MAX`].
	pub fn sequence_number(&self) -> u32 {
		match self {
			Self::InBound(port) => port.sequence_number(),
			Self::InOutBound(port) => port.sequence_number(),
			Self::OutBound(port) => port.sequence_number(),
		}
	}

	/// Returns the T, removing it from the port.
	pub fn take<T: AnyPortValue>(&mut self) -> Result<Option<T>, Error> {
		match self {
			Self::InOutBound(port) => port.take(),
			Self::InBound(_) | Self::OutBound(_) => Err(Error::PortType),
		}
	}

	/// Sets a new value to the T.
	pub fn set<T: AnyPortValue>(&mut self, value: T) -> Result<(), Error> {
		match self {
			Self::OutBound(port) => port.set(value),
			Self::InOutBound(port) => port.set(value),
			Self::InBound(_) => Err(Error::PortType),
		}
	}

	pub fn write<T: AnyPortValue>(&mut self) -> Result<PortValueWriteGuard<T>, Error> {
		match self {
			Self::OutBound(port) => port.write(),
			Self::InOutBound(port) => port.write(),
			Self::InBound(_) => Err(Error::PortType),
		}
	}

	pub fn try_write<T: AnyPortValue>(&mut self) -> Result<PortValueWriteGuard<T>, Error> {
		match self {
			Self::OutBound(port) => port.try_write(),
			Self::InOutBound(port) => port.try_write(),
			Self::InBound(_) => Err(Error::PortType),
		}
	}

	/// Returns the T, removing it from the port.
	pub fn into_inner<T: AnyPortValue>(self) -> Result<Option<T>, Error> {
		match self {
			Self::InBound(port) => port.into_inner(),
			Self::InOutBound(port) => port.into_inner(),
			Self::OutBound(port) => port.into_inner(),
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
