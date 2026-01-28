// Copyright © 2026 Stephan Kunz
//! Test empty port array

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortArray, PortCollection, PortCollectionAccessors,
	PortCollectionAccessorsMut, PortProvider, PortVariant, create_port_array,
};

struct WithPortArray {
	field: i32,
	portlist: PortArray<0>,
}

impl WithPortArray {
	pub fn provided_ports(&self) -> &impl PortCollectionAccessors {
		&self.portlist
	}

	pub fn provided_ports_mut(&mut self) -> &mut impl PortCollectionAccessorsMut {
		&mut self.portlist
	}

	pub fn port_collection(&self) -> &impl PortCollection {
		&self.portlist
	}

	//pub fn port_provider(&mut self) -> &mut impl PortProvider {
	//	&mut self.portlist
	//}
}

struct WithoutPortArray {
	field: i32,
}

impl WithoutPortArray {
	pub fn provided_ports(&self) -> &impl PortCollectionAccessors {
		&dataport::EMPTY_PORT_ARRAY
	}

	// @TODO: return should be &mut !!
	pub fn provided_ports_mut(&mut self) -> &impl PortCollectionAccessorsMut {
		&dataport::EMPTY_PORT_ARRAY
	}

	pub fn port_collection(&self) -> &impl PortCollection {
		&dataport::EMPTY_PORT_ARRAY
	}

	//pub fn port_provider(&mut self) -> &mut impl PortProvider {
	//	&mut dataport::EMPTY_PORT_ARRAY
	//}
}

#[test]
fn array_empty_manual() {
	let st = WithPortArray {
		field: 42,
		portlist: PortArray::from([]),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());

	let wo = WithoutPortArray { field: 42 };

	assert!(wo.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn array_empty_function() {
	let st = WithPortArray {
		field: 42,
		portlist: PortArray::empty(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn array_empty_macro() {
	let st = WithPortArray {
		field: 42,
		portlist: create_port_array!(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}
