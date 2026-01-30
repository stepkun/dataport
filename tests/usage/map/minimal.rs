// Copyright © 2026 Stephan Kunz
//! Test minimal port array

#![allow(missing_docs)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut,
	PortCollectionMut, PortCollectionProvider, PortCollectionProviderMut, PortMap, PortVariant, create_inbound_entry,
	create_inoutbound_entry, create_outbound_entry, create_port_map,
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
fn map_minimal_manual() {
	let mut st = WithPortMap {
		field: 42,
		port_collection: PortMap::from_array([
			("in".into(), PortVariant::InBound(BoundInPort::new::<i32>())),
			("inout".into(), PortVariant::InOutBound(BoundInOutPort::new::<i32>())),
			("out".into(), PortVariant::OutBound(BoundOutPort::new::<i32>())),
		]),
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
	assert!(st.provided_ports().get::<i32>("in").is_ok());
	assert!(st.provided_ports().get::<i32>("inout").is_ok());
	assert!(st.provided_ports().get::<i32>("out").is_err());
	assert!(
		st.provided_ports_mut()
			.set::<i32>("out", 42)
			.is_ok()
	);
}

#[test]
fn map_minimal_macro() {
	let mut st = WithPortMap {
		field: 42,
		port_collection: create_port_map![
			create_inbound_entry!("in", i32),
			create_inoutbound_entry!("inout", i32),
			create_outbound_entry!("out", i32),
		],
	};

	assert!(st.provided_ports().get::<i32>("test").is_err());
	assert!(st.provided_ports().get::<i32>("in").is_ok());
	assert!(st.provided_ports().get::<i32>("inout").is_ok());
	assert!(st.provided_ports().get::<i32>("out").is_err());
	assert!(
		st.provided_ports_mut()
			.set::<i32>("out", 42)
			.is_ok()
	);
}
