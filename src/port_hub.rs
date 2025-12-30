// Copyright © 2025 Stephan Kunz
//! Dynamic list of ports.

use core::ops::{Deref, DerefMut};

use alloc::vec::Vec;

use crate::{PortAccessors, PortCommons, PortProvider, port::Port};

/// PortHub.
#[derive(Default)]
pub struct PortHub(Vec<Port>);

impl Deref for PortHub {
	type Target = [Port];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for PortHub {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl PortAccessors for PortHub {}

impl PortProvider for PortHub {
	fn find(&self, name: impl Into<crate::ConstString>) -> Option<&Port> {
		let name = name.into();
		self.0
			.iter()
			.find(|&port| port.name() == name)
			.map(|v| v as _)
	}
}

impl PortHub {
	pub fn new(ports: Vec<Port>) -> Self {
		Self(ports)
	}

	/// Adds a port to the portlist.
	pub fn add(&mut self, port: Port) {
		self.0.push(port)
	}

	/// Removes a port from the port list.
	pub fn remove(&mut self, name: &str) -> Option<Port> {
		let index = self
			.0
			.iter()
			.position(|port| port.name() == name.into());
		index.map(|index| self.0.remove(index))
	}
}

#[cfg(test)]
mod tests {
	use alloc::{string::String, vec};

	use super::*;

	use crate::traits::PortProvider;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&PortHub>();
		is_normal::<PortHub>();
	}

	const CONST_NAME: &str = "p2";
	static STATIC_NAME: &str = "p3";

	// test constructors.
	#[test]
	fn constructors() {
		let _s0 = PortHub::new(vec![]);
		let _s1 = PortHub::new(vec![Port::create_in_port::<i32>("p1")]);
		let _s2 = PortHub::new(vec![
			Port::create_in_port::<i32>("p1"),
			Port::create_in_port::<f64>(CONST_NAME),
		]);
		let _s3 = PortHub::new(vec![
			Port::create_in_port::<i32>("p1"),
			Port::create_in_port::<f64>(CONST_NAME),
			Port::create_in_port::<String>(STATIC_NAME),
		]);
	}

	// test constructors.
	#[test]
	fn find() {
		let s = PortHub::new(vec![
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
