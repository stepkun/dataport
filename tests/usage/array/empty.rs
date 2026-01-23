// Copyright © 2026 Stephan Kunz
//! Test empty port array

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortArray, PortCollection, PortCollectionAccessors, PortProvider,
	PortVariant, port_array,
};

struct WithPortArray {
	field: i32,
	portlist: PortArray<0>,
}

impl WithPortArray {
	pub fn provided_ports(&self) -> &impl PortProvider {
		&self.portlist
	}
}

struct WithoutPortArray {
	field: i32,
}

impl WithoutPortArray {
	pub fn provided_ports(&self) -> &impl PortProvider {
		&dataport::EMPTY_PORT_ARRAY
	}
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
		portlist: port_array!(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}
