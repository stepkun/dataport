// Copyright © 2026 Stephan Kunz
//! Test port list for const struct

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, Error, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut,
	PortCollectionMut, PortList, PortMap, PortProvider, PortProviderMut, PortVariant, create_inbound_entry,
	create_inoutbound_entry, create_outbound_entry, create_port_list,
};

struct WithPortList<const C: usize> {
	size: usize,
	field: i32,
	portlist: PortList,
}

impl<const C: usize> WithPortList<C> {
	pub fn new(field: i32, portlist: PortList) -> Self {
		Self {
			size: portlist.len(),
			field,
			portlist,
		}
	}
}

impl<const C: usize> WithPortList<C> {
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
fn list_const_manual() {
	let mut st = WithPortList::<3> {
		size: 3,
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
fn list_const_function() {
	let mut st = WithPortList::<3>::new(
		42,
		PortList::from([
			("in".into(), PortVariant::InBound(BoundInPort::new::<i32>())),
			("inout".into(), PortVariant::InOutBound(BoundInOutPort::new::<i32>())),
			("out".into(), PortVariant::OutBound(BoundOutPort::new::<i32>())),
		]),
	);
}

#[test]
fn list_const_macro() {
	let mut st = WithPortList::<3> {
		size: 3,
		field: 42,
		portlist: create_port_list![
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

	assert_eq!(st.provided_ports_mut().set::<i32>("in", 41), Err(Error::WrongPortType));
	assert_eq!(st.provided_ports_mut().set("in", 42), Err(Error::WrongPortType));
	assert_eq!(st.port_provider_mut().remove::<f64>("in"), Err(Error::WrongDataType));
	assert_eq!(st.port_provider_mut().remove::<i32>("in"), Ok(None));
	assert_eq!(
		st.port_provider_mut().remove::<i32>("test"),
		Err(Error::NotFound { name: "test".into() })
	);
}
