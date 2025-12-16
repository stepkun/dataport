// Copyright © 2025 Stephan Kunz
//! Test the port concept

#![allow(unused)]
#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use dataport::{DynamicPortList, Port, PortHub, PortList, StaticPortList};

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

struct BasicStruct {
	other_field: i32,
	ports: StaticPortList<3>,
}

#[test]
/// Things that have ports 'declare' their ports statically.
fn declaration() {
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
fn dynamic_provision() {
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
