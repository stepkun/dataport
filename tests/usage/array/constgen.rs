// Copyright © 2026 Stephan Kunz
//! Test port array for const struct

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, PortArray, PortCollectionAccessors, PortProvider, PortVariant,
	create_inbound_entry, create_inoutbound_entry, create_outbound_entry, create_port_array,
};

struct WithPortArray<const C: usize> {
	size: usize,
	field: i32,
	portlist: PortArray<C>,
}

impl<const C: usize> WithPortArray<C> {
	pub fn new(field: i32, portlist: PortArray<C>) -> Self {
		Self {
			size: C,
			field,
			portlist,
		}
	}

	pub fn provided_ports(&self) -> &impl PortProvider {
		&self.portlist
	}

	pub fn provided_ports_mut(&mut self) -> &mut impl PortProvider {
		&mut self.portlist
	}
}

#[test]
fn array_const_manual() {
	let mut st = WithPortArray::<3> {
		size: 3,
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
fn array_const_function() {
	let mut st = WithPortArray::new(
		42,
		PortArray::from([
			("in".into(), PortVariant::InBound(BoundInPort::new::<i32>())),
			("inout".into(), PortVariant::InOutBound(BoundInOutPort::new::<i32>())),
			("out".into(), PortVariant::OutBound(BoundOutPort::new::<i32>())),
		]),
	);
}

#[test]
fn array_const_macro() {
	let mut st = WithPortArray::<3> {
		size: 3,
		field: 42,
		portlist: create_port_array![
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
