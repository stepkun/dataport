// Copyright © 2025 Stephan Kunz
//! dataport errors.

/// Shortcut for [`dataport`](crate)'s Result<T, E> type
pub type Result<T> = core::result::Result<T, Error>;

/// Dataport error.
#[non_exhaustive]
pub enum Error {
	/// Port is currently locked.
	IsLocked {
		/// Name of the port.
		port: &'static str,
	},
	/// No source for the value of a port set.
	NoSrcSet {
		/// Name of the port.
		port: &'static str,
	},
	/// Port not found.
	NotFound {
		/// Name of the port.
		port: &'static str,
	},
	/// No default value defined for a port.
	NoValueSet {
		/// Name of the port.
		port: &'static str,
	},
	/// Source of input port already set.
	SrcAlreadySet {
		/// Name of the port.
		port: &'static str,
	},
	/// Port has another type than wanted.
	WrongType {
		/// Name of the port.
		port: &'static str,
	},
}

// Only default implementation needed.
impl core::error::Error for Error {}

impl core::fmt::Debug for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::IsLocked { port } => write!(f, "IsLocked(port: {port})"),
			Self::NoSrcSet { port } => write!(f, "NoSrcSet(port: {port})"),
			Self::NotFound { port } => write!(f, "NotFound(port: {port})"),
			Self::NoValueSet { port } => write!(f, "NoValueSet(port: {port})"),
			Self::SrcAlreadySet { port } => write!(f, "SrcAlreadySet(port: {port})"),
			Self::WrongType { port } => write!(f, "WrongType(port: {port})"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::IsLocked { port } => write!(f, "port '{port}' is currently locked"),
			Self::NoSrcSet { port } => write!(f, "no source set for value of port '{port}'"),
			Self::NotFound { port } => write!(f, "port '{port}' was not found"),
			Self::NoValueSet { port } => write!(f, "no value set in port '{port}'"),
			Self::SrcAlreadySet { port } => write!(f, "source of input port '{port}' is already set"),
			Self::WrongType { port } => write!(f, "port: '{port}' has not the wanted type"),
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
		is_normal::<&Error>();
		is_normal::<Error>();
	}
}
