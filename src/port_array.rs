// Copyright © 2025 Stephan Kunz
//! Static array of ports.

use core::ops::Deref;

use crate::{
	port::Port,
	traits::{PortAccessors, PortCommons, PortProvider},
};

/// An array like container for [`Port`]s.
#[repr(transparent)]
pub struct PortArray<const S: usize>([Port; S]);

impl<const S: usize> core::fmt::Debug for PortArray<S> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("PortArray").field(&self.0).finish()
	}
}

impl<const S: usize> Deref for PortArray<S> {
	type Target = [Port];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<const S: usize> PortAccessors for PortArray<S> {}

impl<const S: usize> PortProvider for PortArray<S> {
	fn find(&self, name: impl Into<crate::ConstString>) -> Option<&Port> {
		let name = name.into();
		self.0
			.iter()
			.find(|&port| port.name() == name)
			.map(|v| v as _)
	}
}

impl<const S: usize> PortArray<S> {
	pub fn new(ports: [Port; S]) -> Self {
		Self(ports)
	}
}

#[cfg(test)]
mod tests {
	use alloc::string::String;

	use super::*;

	use crate::traits::PortProvider;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&PortArray<2>>();
		is_normal::<PortArray<4>>();
	}

	const CONST_NAME: &str = "p2";
	static STATIC_NAME: &str = "p3";

	// test constructors.
	#[test]
	fn constructors() {
		let _s0 = PortArray::new([]);
		let _s1 = PortArray::new([Port::create_in_port::<i32>("p1")]);
		let _s2 = PortArray::new([
			Port::create_in_port::<i32>("p1"),
			Port::create_in_port::<f64>(CONST_NAME),
		]);
		let _s3 = PortArray::new([
			Port::create_in_port::<i32>("p1"),
			Port::create_in_port::<f64>(CONST_NAME),
			Port::create_in_port::<String>(STATIC_NAME),
		]);
	}

	// test constructors.
	#[test]
	fn find() {
		let s = PortArray::new([
			Port::create_in_port::<i32>("p1"),
			Port::create_in_port::<f64>(CONST_NAME),
			Port::create_in_port::<String>(STATIC_NAME),
		]);

		assert!(s.find("p1").is_some());
		assert!(s.find(CONST_NAME).is_some());
		assert!(s.find("p2").is_some());
		assert_eq!(s.find("p2"), s.find(CONST_NAME));
		assert!(s.find(STATIC_NAME).is_some());
		assert!(s.find("p3").is_some());
		assert_eq!(s.find("p3"), s.find(STATIC_NAME));
		assert!(s.find("p_non_existent").is_none());
	}
}
