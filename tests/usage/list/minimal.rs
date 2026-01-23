// Copyright © 2026 Stephan Kunz
//! Test minimal port array

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortCollectionAccessors, PortList, PortProvider, PortVariant, inbound,
	inoutbound, outbound, port_list,
};

struct WithPortList {
	field: i32,
	portlist: PortList,
}

impl WithPortList {
	pub fn provided_ports(&self) -> &impl PortProvider {
		&self.portlist
	}

	pub fn provided_ports_mut(&mut self) -> &mut impl PortProvider {
		&mut self.portlist
	}
}

#[test]
fn list_minimal_manual() {
	let mut st = WithPortList {
		field: 42,
		portlist: PortList::from([
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
	let mut st = WithPortList {
		field: 42,
		portlist: port_list![
			inbound!("in", i32),
			inoutbound!("inout", i32),
			outbound!("out", i32),
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
