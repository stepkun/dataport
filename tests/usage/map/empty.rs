// Copyright © 2026 Stephan Kunz
//! Test empty port list

#![allow(missing_docs)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut, PortMap,
	PortProvider, PortVariant, create_port_map,
};

struct WithPortMap {
	field: i32,
	portlist: PortMap,
}

impl WithPortMap {
	pub fn provided_ports(&self) -> &impl PortCollectionAccessors {
		&self.portlist
	}

	pub fn provided_ports_mut(&mut self) -> &mut impl PortCollectionAccessorsMut {
		&mut self.portlist
	}

	pub fn port_collection(&self) -> &impl PortCollection {
		&self.portlist
	}

	pub fn port_provider(&mut self) -> &mut impl PortProvider {
		&mut self.portlist
	}
}

#[test]
fn map_empty_manual() {
	let st = WithPortMap {
		field: 42,
		portlist: PortMap::from([]),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn map_empty_function() {
	let st = WithPortMap {
		field: 42,
		portlist: PortMap::default(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn map_empty_macro() {
	let st = WithPortMap {
		field: 42,
		portlist: create_port_map!(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}
