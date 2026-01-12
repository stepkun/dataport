// Copyright © 2026 Stephan Kunz
//! A flowing input type port implementing [`FlowIn`].

use crate::flow::{FlowCommons, FlowIn};

/// @TODO:
#[derive(Clone, Debug)]
pub struct FlowingInPort(u32);

impl FlowingInPort {
	pub fn new(value: u32) -> Self {
		Self(value)
	}
}

impl FlowCommons for FlowingInPort {}

impl FlowIn for FlowingInPort {}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&FlowingInPort>();
		is_normal::<FlowingInPort>();
	}
}
