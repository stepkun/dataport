// Copyright © 2026 Stephan Kunz
//! Test minimal port array

#![allow(missing_docs)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortArray, PortCollection, PortCollectionAccessors,
	PortCollectionAccessorsMut, PortProvider, PortVariant, create_inbound_entry, create_inoutbound_entry,
	create_outbound_entry, create_port_array,
};

struct WithPortArray {
	field: i32,
	portlist: PortArray<3>,
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

#[test]
fn array_minimal_manual() {
	let mut st = WithPortArray {
		field: 42,
		portlist: PortArray::from([
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
fn array_minimal_macro() {
	let mut st = WithPortArray {
		field: 42,
		portlist: create_port_array!(
			create_inbound_entry!("in", i32),
			create_inoutbound_entry!("inout", i32),
			create_outbound_entry!("out", i32),
		),
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
