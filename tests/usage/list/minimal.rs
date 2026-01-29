// Copyright © 2026 Stephan Kunz
//! Test minimal port array

#![allow(missing_docs)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut,
	PortCollectionMut, PortCollectionProvider, PortCollectionProviderMut, PortVariant, PortVec, create_inbound_entry,
	create_inoutbound_entry, create_outbound_entry, create_port_vec,
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
fn list_minimal_manual() {
	let mut st = WithPortVec {
		field: 42,
		port_collection: PortVec::from([
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
fn list_minimal_macro() {
	let mut st = WithPortVec {
		field: 42,
		port_collection: create_port_vec![
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
