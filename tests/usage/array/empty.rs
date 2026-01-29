// Copyright © 2026 Stephan Kunz
//! Test empty port array

#![allow(missing_docs)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortArray, PortCollection, PortCollectionAccessors,
	PortCollectionAccessorsMut, PortCollectionMut, PortCollectionProvider, PortVariant, create_port_array,
};

struct WithPortArray {
	field: i32,
	port_collection: PortArray<0>,
}

impl WithPortArray {}

impl PortCollectionProvider for WithPortArray {
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

struct WithoutPortArray {
	field: i32,
}

impl WithoutPortArray {
	pub fn provided_ports(&self) -> &impl PortCollectionAccessors {
		unsafe { &*dataport::EMPTY_PORT_ARRAY }
	}
}

impl PortCollectionProvider for WithoutPortArray {
	fn provided_ports(&self) -> &impl PortCollectionAccessors {
		unsafe { &*dataport::EMPTY_PORT_ARRAY }
	}

	fn provided_ports_mut(&mut self) -> &mut impl PortCollectionAccessorsMut {
		unsafe { &mut *dataport::EMPTY_PORT_ARRAY }
	}

	fn port_collection(&self) -> &impl PortCollection {
		unsafe { &*dataport::EMPTY_PORT_ARRAY }
	}
}

#[test]
fn array_empty_manual() {
	let st = WithPortArray {
		field: 42,
		port_collection: PortArray::from([]),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());

	let wo = WithoutPortArray { field: 42 };

	assert!(wo.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn array_empty_function() {
	let st = WithPortArray {
		field: 42,
		port_collection: PortArray::empty(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn array_empty_macro() {
	let st = WithPortArray {
		field: 42,
		port_collection: create_port_array!(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}
