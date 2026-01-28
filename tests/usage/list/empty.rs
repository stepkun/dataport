// Copyright © 2026 Stephan Kunz
//! Test empty port list

#![allow(missing_docs)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut,
	PortList, PortMap, PortProvider, PortVariant, create_port_list,
};

struct WithPortList {
	field: i32,
	portlist: PortList,
}

impl WithPortList {
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
fn list_empty_manual() {
	let st = WithPortList {
		field: 42,
		portlist: PortList::from([]),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn list_empty_function() {
	let st = WithPortList {
		field: 42,
		portlist: PortList::default(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}

#[test]
fn list_empty_macro() {
	let st = WithPortList {
		field: 42,
		portlist: create_port_list!(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}
