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
}

// Only default implementation needed.
impl core::error::Error for Error {}

impl core::fmt::Debug for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::CouldNotConvert { value, port } => write!(f, "CouldNotConvert(value: {value}, value: {port})"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::CouldNotConvert { value, port } => write!(f, "could not convert '{value}' into wanted type for {port}"),
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
