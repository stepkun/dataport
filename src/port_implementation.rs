// Copyright © 2025 Stephan Kunz
//! Port implementation.

#![allow(unused)]

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		// is_normal::<&PortDescription>();
		// is_normal::<PortDescription>();
	}
}
