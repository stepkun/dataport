// Copyright © 2026 Stephan Kunz
//! Test empty port list

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortCollection, PortCollectionAccessors, PortMap, PortProvider, PortVariant,
};

struct WithPortMap {
	field: i32,
	portlist: PortMap,
}

impl WithPortMap {
	pub fn provided_ports(&self) -> &impl PortProvider {
		&self.portlist
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
#[ignore = "todo!"]
fn map_empty_macro() {
	let st = WithPortMap {
		field: 42,
		portlist: todo!(), //port_map!(),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
}
