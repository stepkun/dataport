// Copyright © 2026 Stephan Kunz
//! Test empty port list

#![allow(missing_docs)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut,
	PortCollectionMut, PortCollectionProvider, PortCollectionProviderMut, PortMap, PortVariant, PortVec, create_port_vec,
};

struct WithPortVec {
	field: i32,
	port_collection: PortVec,
}

impl PortCollectionProvider for WithPortVec {
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

impl PortCollectionProviderMut for WithPortVec {
	fn port_collection_mut(&mut self) -> &mut impl PortCollectionMut {
		&mut self.port_collection
	}
}

#[test]
fn list_empty_manual() {
	let st = WithPortVec {
		field: 42,
		port_collection: PortVec::from([]),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn list_empty_function() {
	let st = WithPortVec {
		field: 42,
		port_collection: PortVec::default(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn list_empty_macro() {
	let st = WithPortVec {
		field: 42,
		port_collection: create_port_vec!(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}
