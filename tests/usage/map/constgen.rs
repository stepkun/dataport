// Copyright © 2026 Stephan Kunz
//! Test port list for const struct

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, Error, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut,
	PortCollectionMut, PortMap, PortProvider, PortProviderMut, PortVariant, create_inbound_entry, create_inoutbound_entry,
	create_outbound_entry, create_port_map,
};

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

	pub fn provided_ports_mut(&mut self) -> &mut impl PortProviderMut {
		&mut self.portlist
	}

	pub fn port_provider(&self) -> &impl PortCollection {
		&self.portlist
	}

	pub fn port_provider_mut(&mut self) -> &mut impl PortCollectionMut {
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

	assert_eq!(
		st.provided_ports().get::<i32>("test"),
		Err(Error::NotFound { name: "test".into() })
	);
	assert_eq!(st.provided_ports().get::<i32>("in"), Ok(None));
	assert_eq!(st.provided_ports().get::<i32>("inout"), Ok(None));
	assert_eq!(st.provided_ports().get::<i32>("out"), Err(Error::WrongPortType));
	assert_eq!(st.provided_ports_mut().set::<i32>("out", 42), Ok(()));
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
fn map_const_macro() {
	let mut st = WithPortMap::<3> {
		size: 3,
		field: 42,
		portlist: create_port_map![
			create_inbound_entry!("in", i32),
			create_inoutbound_entry!("inout", i32),
			create_outbound_entry!("out", i32),
		],
	};

	assert_eq!(
		st.provided_ports().get::<i32>("test"),
		Err(Error::NotFound { name: "test".into() })
	);
	assert_eq!(st.provided_ports().get::<i32>("in"), Ok(None));
	assert_eq!(st.provided_ports().get::<i32>("inout"), Ok(None));
	assert_eq!(st.provided_ports().get::<i32>("out"), Err(Error::WrongPortType));
	assert_eq!(st.provided_ports_mut().set::<i32>("out", 41), Ok(()));
	assert_eq!(st.provided_ports_mut().set("out", 42), Ok(()));

	assert!(
		st.provided_ports_mut()
			.set::<i32>("inout", 41)
			.is_ok()
	);
	assert!(st.provided_ports_mut().set("inout", 42).is_ok());
	st.port_provider_mut().remove::<i32>("inout");

	assert_eq!(st.provided_ports_mut().set::<i32>("in", 41), Err(Error::WrongPortType));
	assert_eq!(st.provided_ports_mut().set("in", 42), Err(Error::WrongPortType));
	let x = st.provided_ports().find("in").is_some();
	assert_eq!(st.port_provider_mut().remove::<f64>("in"), Err(Error::WrongDataType));
	let x = st.provided_ports().find("in").is_some();
	assert_eq!(st.port_provider_mut().remove::<i32>("in"), Ok(None));
	assert_eq!(
		st.port_provider_mut().remove::<i32>("test"),
		Err(Error::NotFound { name: "test".into() })
	);
}
