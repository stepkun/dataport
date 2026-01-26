// Copyright © 2026 Stephan Kunz
//! Test empty port list

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortCollection, PortCollectionAccessors, PortCollectionMut, PortMap,
	PortProvider, PortProviderMut, PortVariant, create_port_map,
};

struct WithPortMap {
	field: i32,
	portlist: PortMap,
}

impl WithPortMap {
	pub fn provided_ports(&self) -> &impl PortProvider {
		&self.portlist
	}

	pub fn provided_ports_mut(&mut self) -> &mut impl PortProviderMut {
		&mut self.portlist
	}

	pub fn port_provider(&self) -> &impl PortCollection {
		&self.portlist
	}

	pub fn port_provider_mut(&mut self) -> &mut impl PortCollectionMut {
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
