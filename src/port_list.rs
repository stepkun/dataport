// Copyright © 2025 Stephan Kunz
//! List of ports.

#![allow(unused)]

use core::ops::Deref;
#[cfg(feature = "alloc")]
use core::ops::DerefMut;

use alloc::vec::Vec;

use crate::port::Port;

/// PortList.
pub trait PortList {
	/// Returns a reference to the port list.
	fn portlist(&self) -> &[Port];

	/// Lookup a [`Port`].
	#[must_use]
	fn find(&self, name: &str) -> Option<&Port> {
		self.portlist()
			.iter()
			.find(|&port| port.name() == name)
			.map(|v| v as _)
	}
}

/// PortHub.
#[cfg(feature = "alloc")]
pub trait PortHub: PortList {
	/// Returns a mutable reference to the port list.
	fn portlist_mut(&mut self) -> &mut Vec<Port>;

	/// Adds a port to the portlist.
	fn add(&mut self, port: Port) {
		self.portlist_mut().push(port)
	}

	/// Removes a port from the port list.
	fn remove(&mut self, name: &str) -> Option<Port> {
		let list = self.portlist_mut();
		let index = list.iter().position(|port| port.name() == name);
		index.map(|index| list.remove(index))
	}
}

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
#[cfg(feature = "alloc")]
#[derive(Default)]
pub struct DynamicPortList(Vec<Port>);

#[cfg(feature = "alloc")]
impl Deref for DynamicPortList {
	type Target = [Port];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[cfg(feature = "alloc")]
impl DerefMut for DynamicPortList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

#[cfg(feature = "alloc")]
impl PortList for DynamicPortList {
	fn portlist(&self) -> &[Port] {
		&self.0
	}
}

#[cfg(feature = "alloc")]
impl PortHub for DynamicPortList {
	fn portlist_mut(&mut self) -> &mut Vec<Port> {
		&mut self.0
	}
}

#[cfg(feature = "alloc")]
impl DynamicPortList {
	pub fn new(ports: Vec<Port>) -> Self {
		Self(ports)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

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
		let s0 = StaticPortList::new([]);
		let s1 = StaticPortList::new([Port::new("p1")]);
		let s2 = StaticPortList::new([Port::new("p1"), Port::new(CONST_NAME)]);
		let s3 = StaticPortList::new([
			Port::new("p1"),
			Port::new(CONST_NAME),
			Port::new(STATIC_NAME),
		]);
	}

	// test constructors.
	#[test]
	fn find() {
		// static list
		{
			let s = StaticPortList::new([
				Port::new("p1"),
				Port::new(CONST_NAME),
				Port::new(STATIC_NAME),
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
		#[cfg(feature = "alloc")]
		{
			use alloc::vec;

			let s = DynamicPortList::new(vec![
				Port::new("p1"),
				Port::new(CONST_NAME),
				Port::new(STATIC_NAME),
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
}
