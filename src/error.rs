// Copyright © 2026 Stephan Kunz
//! Error implementation.

/// Port errors.
#[derive(PartialEq)]
#[non_exhaustive]
#[repr(C)]
pub enum Error {
	/// A port with the given name is already in the collection.
	AlreadyInCollection,
	/// A port cannot be found in a port collection.
	NotFound,
	/// The port 'other' cannot be found in a port collection.
	OtherNotFound,
	/// A ports value is currently locked.
	IsLocked,
	/// No value set for a port.
	NoValueSet,
	/// A port has other data type then expected.
	DataType,
	/// A port is not the needed type.
	PortType,
}

/// Only default implementation needed.
impl core::error::Error for Error {}

impl core::fmt::Debug for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::AlreadyInCollection => write!(f, "a port with that name is already in the collection"),
			Self::IsLocked => write!(f, "port is currently locked"),
			Self::NotFound => write!(f, "port 'self' could not be found in the collection"),
			Self::OtherNotFound => write!(f, "port 'other' could not be found in the collection"),
			Self::NoValueSet => write!(f, "no value set for port"),
			Self::DataType => write!(f, "port has a different data type then expected"),
			Self::PortType => write!(f, "port has an incompatible type"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::fmt::Debug::fmt(self, f)
	}
}
