// Copyright © 2026 Stephan Kunz
//! Test port list for const struct

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{BoundInOutPort, BoundInPort, BoundOutPort, PortCollectionAccessors, PortMap, PortProvider, PortVariant};

struct WithPortMap<const C: usize> {
	size: usize,
	field: i32,
	portlist: PortMap,
}

impl<const C: usize> WithPortMap<C> {
	pub fn new(field: i32, portlist: PortMap) -> Self {
		Self {
			size: portlist.len(),
			field,
			portlist,
		}
	}
}

impl<const C: usize> WithPortMap<C> {
	pub fn provided_ports(&self) -> &impl PortProvider {
		&self.portlist
	}

	pub fn provided_ports_mut(&mut self) -> &mut impl PortProvider {
		&mut self.portlist
	}
}

#[test]
fn map_const_manual() {
	let mut st = WithPortMap::<3> {
		size: 3,
		field: 42,
		portlist: PortMap::from([
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
fn map_const_function() {
	let mut st = WithPortMap::<3>::new(
		42,
		PortMap::from([
			("in".into(), PortVariant::InBound(BoundInPort::new::<i32>())),
			("inout".into(), PortVariant::InOutBound(BoundInOutPort::new::<i32>())),
			("out".into(), PortVariant::OutBound(BoundOutPort::new::<i32>())),
		]),
	);
}

#[test]
#[ignore = "todo!"]
fn map_const_macro() {
	let mut st = WithPortMap::<3> {
		size: 3,
		field: 42,
		portlist: todo!(), /*port_map![
							   inbound!("in", i32),
							   inoutbound!("inout", i32>),
							   outbound!("out", i32),
						   ],*/
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
