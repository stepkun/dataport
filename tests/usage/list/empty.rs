// Copyright © 2026 Stephan Kunz
//! Test empty port list

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortMap, PortCollection, PortCollectionAccessors, PortList, PortProvider,
	PortVariant,
};

struct WithPortList {
	field: i32,
	portlist: PortList,
}

impl WithPortList {
	pub fn provided_ports(&self) -> &impl PortProvider {
		&self.portlist
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
#[ignore = "todo!"]
fn list_empty_macro() {
	let st = WithPortList {
		field: 42,
		portlist: todo!(), //port_list!(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}
