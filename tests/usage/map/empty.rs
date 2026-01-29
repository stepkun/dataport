// Copyright © 2026 Stephan Kunz
//! Test empty port list

#![allow(missing_docs)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut,
	PortCollectionMut, PortCollectionProvider, PortCollectionProviderMut, PortMap, PortVariant, create_port_map,
};

struct WithPortMap {
	field: i32,
	port_collection: PortMap,
}

impl PortCollectionProvider for WithPortMap {
	fn provided_ports(&self) -> &impl PortCollectionAccessors {
		&self.port_collection
	}

	fn provided_ports_mut(&mut self) -> &mut impl PortCollectionAccessorsMut {
		&mut self.port_collection
	}

	fn port_collection(&self) -> &impl PortCollection {
		&self.port_collection
	}
}

impl PortCollectionProviderMut for WithPortMap {
	fn port_collection_mut(&mut self) -> &mut impl PortCollectionMut {
		&mut self.port_collection
	}
}

#[test]
fn map_empty_manual() {
	let st = WithPortMap {
		field: 42,
		port_collection: PortMap::from([]),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn map_empty_function() {
	let st = WithPortMap {
		field: 42,
		port_collection: PortMap::default(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn map_empty_macro() {
	let st = WithPortMap {
		field: 42,
		port_collection: create_port_map!(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}
