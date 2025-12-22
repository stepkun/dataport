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
			Port::create_in_port::<i32>("p1"),
			Port::create_inout_port::<f64>(CONST_NAME),
			Port::create_out_port::<String>(STATIC_NAME),
		]),
	};

	let s2 = BasicStruct {
		other_field: 2,
		ports: StaticPortList::new([
			Port::create_out_port::<String>(STATIC_NAME),
			Port::create_inout_port::<f64>(CONST_NAME),
			Port::create_in_port::<i32>("p1"),
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

	s1.ports.add(Port::create_in_port::<i32>("p1"));
	s1.ports
		.add(Port::create_inout_port::<f64>(CONST_NAME));
	s1.ports
		.add(Port::create_out_port::<String>(STATIC_NAME));

	let s2 = DynamicStruct {
		other_field: 1,
		ports: DynamicPortList::new(vec![
			Port::create_out_port::<String>(STATIC_NAME),
			Port::create_inout_port::<f64>(CONST_NAME),
			Port::create_in_port::<i32>("p1"),
		]),
	};

	assert_eq!(s1.ports.find("p1"), s2.ports.find("p1"));
	assert_eq!(s1.ports.find("p2"), s2.ports.find("p2"));
	assert_eq!(s1.ports.find("p2"), s2.ports.find(CONST_NAME));
	assert_eq!(s1.ports.find("p3"), s2.ports.find("p3"));
	assert_eq!(s1.ports.find("p3"), s2.ports.find(STATIC_NAME));
	assert_ne!(s1.ports.find("p1"), s2.ports.find("p3"));
}

#[test]
/// Ports are partial equal, if their name & type is equal.
fn port_equality() {
	let p1a = Port::create_in_port::<i32>("p1");
	let p1b = Port::create_in_port::<i32>("p1");
	let p1c = Port::create_in_port::<f64>("p1");
	let p2a = Port::create_in_port::<i32>("p2");
	let p2b = Port::create_in_port::<i32>("p2");
	let p2c = Port::create_in_port::<f64>("p2");

	assert_eq!(p1a, p1b);
	assert_ne!(p1a, p1c);
	assert_ne!(p1a, p2a);
	assert_ne!(p1a, p2b);
	assert_ne!(p1a, p2c);
	assert_eq!(p2a, p2b);
	assert_ne!(p2a, p2c);
}

#[test]
#[ignore]
fn getter_and_setter() {
	let portlist1 = StaticPortList::new([
		Port::create_out_port::<i32>("p1a"),
		Port::create_out_port::<String>("p1b"),
		Port::create_out_port::<f64>("p1c"),
		Port::create_inout_port::<f64>("p1d"),
	]);

	let portlist2 = StaticPortList::new([
		Port::create_in_port::<i32>("p2a"),
		Port::create_in_port::<String>("p2b"),
		Port::create_inout_port::<f64>("p2c"),
		Port::create_in_port::<f64>("p2d"),
	]);

	let res = portlist1.connect_ports::<i32>("p1a", &portlist2, "p2a");
	assert!(res.is_ok());

	let res = portlist1.connect_ports::<String>("p1b", &portlist2, "p2b");
	assert!(res.is_ok());

	let res = portlist1.connect_ports::<f64>("p1c", &portlist2, "p2c");
	assert!(res.is_ok());

	let res = portlist2.connect_ports::<f64>("p2c", &portlist1, "p1d");
	assert!(res.is_ok());

	let res = portlist1.connect_ports::<f64>("p1d", &portlist2, "p2d");
	assert!(res.is_ok());

	portlist1.set("p1a", 42).is_ok();
	assert_eq!(portlist2.get::<i32>("p2a").unwrap().unwrap(), 42);

	portlist1
		.set("p1b", String::from("hello world"))
		.is_ok();
	assert_eq!(portlist2.get::<String>("p2b").unwrap().unwrap(), String::from("hello world"));

	assert!(portlist1.set("p1c", PI).is_ok());
	assert_eq!(portlist2.get::<f64>("p2c").unwrap().unwrap(), PI);
	assert!(portlist2.propagate::<f64>("p2c").is_ok());
	assert_eq!(portlist1.get::<f64>("p1d").unwrap().unwrap(), PI);
	assert!(portlist1.propagate::<f64>("p1d").is_ok());
	assert_eq!(portlist2.get::<f64>("p2d").unwrap().unwrap(), PI);
}
