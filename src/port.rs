// Copyright © 2025 Stephan Kunz
//! Port.

#![allow(unused)]

/// InputPort.
pub trait InputPort {
	/// Returns a reference to the T.
	fn as_ref<T>(&self) -> &T;

	/// Returns a clone/copy of the T.
	fn get<T>(&self) -> T
	where
		T: Clone;
}

/// OutputPort.
pub trait OutputPort {
	/// Returns a mutable reference to the T.
	fn as_mut<T>(&mut self) -> &mut T;

	/// Sets a new value to the T and returns the old T.
	fn set<T>(&mut self, value: T) -> T;
}

/// Port.
#[derive(Debug, Eq, PartialEq)]
pub struct Port {
	name: &'static str,
}

impl Port {
	pub fn new(name: &'static str) -> Self {
		Self { name }
	}

	pub fn name(&self) -> &str {
		self.name
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&Port>();
		is_normal::<Port>();
	}

	const CONST_NAME: &str = "p2";
	static STATIC_NAME: &str = "p3";

	// test constructor
	fn constructor() {
		let p1 = Port::new("p1");
		let p2 = Port::new(CONST_NAME);
		let p3 = Port::new(STATIC_NAME);
	}
}
