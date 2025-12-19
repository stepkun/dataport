// Copyright © 2025 Stephan Kunz
//! dataport errors.

/// Dataport error.
#[non_exhaustive]
pub enum Error {
	/// Could not convert the str into required type of dataport.
	CouldNotConvert {
		/// The value, that cannot be converted.
		value: &'static str,
		/// Name of the port.
		port: &'static str,
	},
	/// No default value defined for a Port.
	NoDefaultDefined {
		/// Name of the port.
		port: &'static str,
	},
	/// No source for the value of a Port set.
	NoSrcSet {
		/// Name of the port.
		port: &'static str,
	},
}

// Only default implementation needed.
impl core::error::Error for Error {}

impl core::fmt::Debug for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::CouldNotConvert { value, port } => write!(f, "CouldNotConvert(value: {value}, port: {port})"),
			Self::NoDefaultDefined { port } => write!(f, "NoDefaultDefined(port: {port})"),
			Self::NoSrcSet { port } => write!(f, "NoSrcSet(port: {port})"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::CouldNotConvert { value, port } => write!(f, "could not convert '{value}' into wanted type for '{port}'"),
			Self::NoDefaultDefined { port } => write!(f, "no default defined for port '{port}'"),
			Self::NoSrcSet { port } => write!(f, "no source set for value of port '{port}'"),
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
