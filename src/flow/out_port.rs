// Copyright © 2026 Stephan Kunz
//! A flowing output type port implementing [`FlowOut`].

use crate::flow::{FlowCommons, FlowOut};

/// @TODO:
#[derive(Clone, Debug)]
pub struct FlowingOutPort(u32);

impl FlowingOutPort {
	pub fn new(value: u32) -> Self {
		Self(value)
	}
}

impl FlowCommons for FlowingOutPort {}

impl FlowOut for FlowingOutPort {}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&FlowingOutPort>();
		is_normal::<FlowingOutPort>();
	}
}
