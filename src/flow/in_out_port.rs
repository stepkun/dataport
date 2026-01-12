// Copyright © 2026 Stephan Kunz
//! A flowing input output type port implementing [`FlowInOut`].

use crate::flow::{FlowCommons, FlowIn, FlowInOut, FlowOut};

/// @TODO:
#[derive(Clone, Debug)]
pub struct FlowingInOutPort(u32);

impl FlowingInOutPort {
	pub fn new(value: u32) -> Self {
		Self(value)
	}
}

impl FlowCommons for FlowingInOutPort {}

impl FlowIn for FlowingInOutPort {}

impl FlowInOut for FlowingInOutPort {}

impl FlowOut for FlowingInOutPort {}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&FlowingInOutPort>();
		is_normal::<FlowingInOutPort>();
	}
}
