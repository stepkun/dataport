// Copyright © 2025 Stephan Kunz
//! Test the port concept

#![allow(unused)]
#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use std::{f64::consts::PI, sync::Arc};

use dataport::{
	DynamicPortList, InOutPort, InPort, OutPort, Port, PortBase, PortGetter, PortHub, PortList, PortSetter, StaticPortList,
};

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

#[test]
fn port_connections() {
	let mut i1 = InPort::<i32>::new("p1");
	let mut i2 = InPort::<f64>::new(CONST_NAME);
	let mut i3 = InPort::<String>::new(STATIC_NAME);

	let mut io1 = InOutPort::<i32>::new("p1");
	let mut io2 = InOutPort::<f64>::new(CONST_NAME);
	let mut io3 = InOutPort::<String>::new(STATIC_NAME);

	let mut o1 = OutPort::<i32>::new("p1");
	let mut o2 = OutPort::<f64>::new(CONST_NAME);
	let mut o3 = OutPort::<String>::new(STATIC_NAME);

	o1.set(42);
	o2.set(PI);
	o3.set(String::from("the answer"));

	io1.set_src(o1);
	io2.set_src(o2);
	io3.set_src(o3);

	// for testing purpose we propagate the value directly to output
	io1.propagate();
	io2.propagate();
	io3.propagate();

	assert_eq!(io1.get().unwrap(), 42);
	assert_eq!(io2.get().unwrap(), PI);
	assert_eq!(io3.get().unwrap(), String::from("the answer"));

	i1.set_src(io1);
	i2.set_src(io2);
	i3.set_src(io3);

	assert_eq!(i1.get().unwrap(), 42);
	assert_eq!(i2.get().unwrap(), PI);
	assert_eq!(i3.get().unwrap(), String::from("the answer"));
}

struct BasicStruct {
	other_field: i32,
	ports: StaticPortList<3>,
}

#[test]
/// Things that have ports 'declare' their ports statically.
fn static_declaration() {
	let s1 = BasicStruct {
		other_field: 1,
		ports: StaticPortList::new([
			Port::new("p1"),
			Port::new(CONST_NAME),
			Port::new(STATIC_NAME),
		]),
	};

	let s2 = BasicStruct {
		other_field: 2,
		ports: StaticPortList::new([
			Port::new(STATIC_NAME),
			Port::new(CONST_NAME),
			Port::new("p1"),
		]),
	};

	assert_eq!(s1.ports.find("p1"), s2.ports.find("p1"));
	assert_eq!(s1.ports.find("p2"), s2.ports.find("p2"));
	assert_eq!(s1.ports.find("p2"), s2.ports.find(CONST_NAME));
	assert_eq!(s1.ports.find("p3"), s2.ports.find("p3"));
	assert_eq!(s1.ports.find("p3"), s2.ports.find(STATIC_NAME));
	assert_ne!(s1.ports.find("p1"), s2.ports.find("p3"));
}

struct DynamicStruct {
	other_field: i32,
	ports: DynamicPortList,
}

#[test]
/// Databases, Blackboards and other communication hubs need to 'provide' ports dynamically.
fn dynamic_provisioning() {
	let mut s1 = DynamicStruct {
		other_field: 1,
		ports: DynamicPortList::default(),
	};
	assert!(s1.ports.find("p1").is_none());
	assert!(s1.ports.find(CONST_NAME).is_none());
	assert!(s1.ports.find(STATIC_NAME).is_none());

	s1.ports.add(Port::new("p1"));
	s1.ports.add(Port::new(CONST_NAME));
	s1.ports.add(Port::new(STATIC_NAME));

	let s2 = DynamicStruct {
		other_field: 1,
		ports: DynamicPortList::new(vec![
			Port::new(STATIC_NAME),
			Port::new(CONST_NAME),
			Port::new("p1"),
		]),
	};

	assert_eq!(s1.ports.find("p1"), s2.ports.find("p1"));
	assert_eq!(s1.ports.find("p2"), s2.ports.find("p2"));
	assert_eq!(s1.ports.find("p2"), s2.ports.find(CONST_NAME));
	assert_eq!(s1.ports.find("p3"), s2.ports.find("p3"));
	assert_eq!(s1.ports.find("p3"), s2.ports.find(STATIC_NAME));
	assert_ne!(s1.ports.find("p1"), s2.ports.find("p3"));
}
