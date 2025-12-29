// Copyright © 2025 Stephan Kunz
//! Static and dynamic list of ports.

use core::ops::{Deref, DerefMut};

use alloc::vec::Vec;

use crate::{
	port::Port,
	traits::{PortHub, PortList},
};

/// StaticPortList.
pub struct StaticPortList<const S: usize>([Port; S]);

impl<const S: usize> Deref for StaticPortList<S> {
	type Target = [Port];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<const S: usize> PortList for StaticPortList<S> {
	fn portlist(&self) -> &[Port] {
		&self.0
	}
}

impl<const S: usize> StaticPortList<S> {
	pub fn new(ports: [Port; S]) -> Self {
		Self(ports)
	}
}

/// DynamicPortList.
#[derive(Default)]
pub struct DynamicPortList(Vec<Port>);

impl Deref for DynamicPortList {
	type Target = [Port];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for DynamicPortList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl PortList for DynamicPortList {
	fn portlist(&self) -> &[Port] {
		&self.0
	}
}

impl PortHub for DynamicPortList {
	fn portlist_mut(&mut self) -> &mut Vec<Port> {
		&mut self.0
	}
}

impl DynamicPortList {
	pub fn new(ports: Vec<Port>) -> Self {
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
		is_normal::<&StaticPortList<2>>();
		is_normal::<StaticPortList<4>>();
		is_normal::<&DynamicPortList>();
		is_normal::<DynamicPortList>();
	}

	const CONST_NAME: &str = "p2";
	static STATIC_NAME: &str = "p3";

	// test constructors.
	#[test]
	fn constructors() {
		let _s0 = StaticPortList::new([]);
		let _s1 = StaticPortList::new([Port::create_in_port::<i32>("p1")]);
		let _s2 = StaticPortList::new([
			Port::create_in_port::<i32>("p1"),
			Port::create_in_port::<f64>(CONST_NAME),
		]);
		let _s3 = StaticPortList::new([
			Port::create_in_port::<i32>("p1"),
			Port::create_in_port::<f64>(CONST_NAME),
			Port::create_in_port::<String>(STATIC_NAME),
		]);
	}

	// test constructors.
	#[test]
	fn find() {
		// static list
		{
			let s = StaticPortList::new([
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
		// dynamic list
		use alloc::vec;

		let s = DynamicPortList::new(vec![
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
