// Copyright © 2026 Stephan Kunz
//! Test port list for const struct

#![allow(missing_docs)]
#![allow(unused)]

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, Error, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut,
	PortCollectionMut, PortCollectionProvider, PortCollectionProviderMut, PortMap, PortVariant, create_inbound_entry,
	create_inoutbound_entry, create_outbound_entry, create_port_map,
};

struct WithPortMap<const C: usize> {
	size: usize,
	field: i32,
	port_collection: PortMap,
}

impl<const C: usize> WithPortMap<C> {
	pub fn new(field: i32, portlist: PortMap) -> Self {
		Self {
			size: portlist.len(),
			field,
			port_collection: portlist,
		}
	}
}

impl<const C: usize> PortCollectionProvider for WithPortMap<C> {
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

impl<const C: usize> PortCollectionProviderMut for WithPortMap<C> {
	fn port_collection_mut(&mut self) -> &mut impl PortCollectionMut {
		&mut self.port_collection
	}
}

#[test]
fn map_const_manual() {
	let mut st = WithPortMap::<3> {
		size: 3,
		field: 42,
		port_collection: PortMap::from_array([
			("in".into(), PortVariant::InBound(BoundInPort::new::<i32>())),
			("inout".into(), PortVariant::InOutBound(BoundInOutPort::new::<i32>())),
			("out".into(), PortVariant::OutBound(BoundOutPort::new::<i32>())),
		]),
	};

	assert_eq!(st.provided_ports().get::<i32>("test"), Err(Error::NotFound));
	assert_eq!(st.provided_ports().get::<i32>("in"), Ok(None));
	assert_eq!(st.provided_ports().get::<i32>("inout"), Ok(None));
	assert_eq!(st.provided_ports().get::<i32>("out"), Err(Error::PortType));
	assert_eq!(st.provided_ports_mut().set::<i32>("out", 42), Ok(()));
}

#[test]
fn map_const_function() {
	let mut st = WithPortMap::<3>::new(
		42,
		PortMap::from_array([
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
		port_collection: create_port_map![
			create_inbound_entry!("in", i32),
			create_inoutbound_entry!("inout", i32),
			create_outbound_entry!("out", i32),
		],
	};

	assert_eq!(st.provided_ports().get::<i32>("test"), Err(Error::NotFound));
	assert_eq!(st.provided_ports().get::<i32>("in"), Ok(None));
	assert_eq!(st.provided_ports().get::<i32>("inout"), Ok(None));
	assert_eq!(st.provided_ports().get::<i32>("out"), Err(Error::PortType));
	assert_eq!(st.provided_ports_mut().set::<i32>("out", 41), Ok(()));
	assert_eq!(st.provided_ports_mut().set("out", 42), Ok(()));

	assert!(
		st.provided_ports_mut()
			.set::<i32>("inout", 41)
			.is_ok()
	);
	assert!(st.provided_ports_mut().set("inout", 42).is_ok());
	st.port_collection_mut().remove::<i32>("inout");

	assert_eq!(st.provided_ports_mut().set::<i32>("in", 41), Err(Error::PortType));
	assert_eq!(st.provided_ports_mut().set("in", 42), Err(Error::PortType));
	let x = st.port_collection().find("in").is_some();
	assert_eq!(st.port_collection_mut().remove::<f64>("in"), Err(Error::DataType));
	let x = st.port_collection().find("in").is_some();
	assert_eq!(st.port_collection_mut().remove::<i32>("in"), Ok(None));
	assert_eq!(st.port_collection_mut().remove::<i32>("test"), Err(Error::NotFound));
}
