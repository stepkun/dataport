// Copyright © 2025 Stephan Kunz
//! Test [`PortHub`] features.

use std::f64::consts::PI;

use dataport::*;

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

struct DynamicStruct {
	_other_field: i32,
	ports: PortList,
}

#[test]
/// Communication hubs need to 'provide' ports dynamically.
fn provisioning() {
	let mut s1 = DynamicStruct {
		_other_field: 1,
		ports: PortList::default(),
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
		_other_field: 1,
		ports: PortList::new(vec![
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
fn bind_get_and_set() {
	let portlist1 = PortList::new(vec![
		Port::create_out_port::<i32>("p1a"),
		Port::create_out_port::<String>("p1b"),
		Port::create_out_port::<f64>("p1c"),
		Port::create_inout_port::<f64>("p1d"),
	]);

	let portlist2 = PortList::new(vec![
		Port::create_in_port::<i32>("p2a"),
		Port::create_in_port::<String>("p2b"),
		Port::create_inout_port::<f64>("p2c"),
		Port::create_in_port::<f64>("p2d"),
	]);

	let res = portlist2.bind_to::<i32>("p2a", &portlist1, "p1a");
	assert!(res.is_ok());

	let res = portlist2.bind_to::<String>("p2b", &portlist1, "p1b");
	assert!(res.is_ok());

	// create p1c -> p2c -> p1d -> p2d
	let res = portlist2.bind_to::<f64>("p2c", &portlist1, "p1c");
	assert!(res.is_ok());
	let res = portlist1.bind_to::<f64>("p1d", &portlist2, "p2c");
	assert!(res.is_ok());
	let res = portlist2.bind_to::<f64>("p2d", &portlist1, "p1d");
	assert!(res.is_ok());

	// test value binding
	assert!(portlist1.set::<i32>("p1a", 42).is_ok());
	assert_eq!(portlist2.get::<i32>("p2a").unwrap(), 42);

	assert!(
		portlist1
			.set::<String>("p1b", String::from("hello world"))
			.is_ok()
	);
	assert_eq!(portlist2.get::<String>("p2b").unwrap(), String::from("hello world"));

	assert!(portlist1.set::<f64>("p1c", PI).is_ok());
	assert_eq!(portlist2.get::<f64>("p2c").unwrap(), PI);
	assert_eq!(portlist1.get::<f64>("p1d").unwrap(), PI);
	assert_eq!(portlist2.get::<f64>("p2d").unwrap(), PI);
}
