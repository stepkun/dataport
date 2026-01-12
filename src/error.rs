// Copyright © 2026 Stephan Kunz
//! Error implementation.

use crate::ConstString;

/// Shortcut for [`dataport`](crate)'s Result<T, E> type
pub(crate) type Result<T> = core::result::Result<T, Error>;

/// Port errors.
#[non_exhaustive]
pub enum Error {
	/// A port with the given name is already in the collection.
	AlreadyInCollection { name: ConstString },
	/// A port cannot be found in a port collection.
	NotFound { name: ConstString },
	/// A ports value is currently locked.
	IsLocked,
	/// No value set for a port.
	NoValueSet,
	/// A port has other data type then expected.
	WrongDataType,
	/// A port is not the needed type.
	WrongPortType,
}

/// Only default implementation needed.
impl core::error::Error for Error {}

impl core::fmt::Debug for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::AlreadyInCollection { name } => write!(f, "AlreadyInCollection('{name}')"),
			Self::NotFound { name } => write!(f, "NotFound('{name}')"),
			Self::IsLocked => write!(f, "IsLocked"),
			Self::NoValueSet => write!(f, "NoValueSet"),
			Self::WrongDataType => write!(f, "WrongDataType"),
			Self::WrongPortType => write!(f, "WrongPortType"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::AlreadyInCollection { name } => write!(f, "a port with the name '{name}' is already in the collection"),
			Self::NotFound { name } => write!(f, "port '{name}' could not be found in the collection"),
			Self::IsLocked => write!(f, "port is currently locked"),
			Self::NoValueSet => write!(f, "no value set for port"),
			Self::WrongDataType => write!(f, "port has a different data type then expected"),
			Self::WrongPortType => write!(f, "port has an incompatible type"),
		}
	}
}
