// Copyright © 2026 Stephan Kunz
//! Test a concept

#![allow(missing_docs)]
#![allow(unused)]

use dataport::{PortCollection, create_inbound_entry, create_inoutbound_entry, create_outbound_entry, create_port_array};

const NAME1: &str = "TEST1";
static NAME2: &str = "TEST2";

#[derive(Clone, Debug)]
struct Test {
	f1: i32,
	f2: f64,
	f3: Vec<String>,
}
impl Test {
	fn new() -> Self {
		Test {
			f1: 42,
			f2: 3.22,
			f3: vec![String::from("test")],
		}
	}
}
#[test]
fn macro_usage() {
	let portlist = create_port_array!(
		create_inbound_entry!("in", i32),
		create_inoutbound_entry!("inout", i32),
		create_outbound_entry!("out", i32)
	);
	let x = portlist.find("in").unwrap();
	let x = portlist.find("inout").unwrap();
	let x = portlist.find("out").unwrap();

	let name1 = "test1";
	let name2 = String::from("test2");

	let portlist = create_port_array!(
		create_inbound_entry!(name1, i32),
		create_inoutbound_entry!(name2, i32),
		create_outbound_entry!("test3", i32)
	);
	let x = portlist.find("test1").unwrap();
	let x = portlist.find("test2").unwrap();
	let x = portlist.find("test3").unwrap();

	let portlist = create_port_array!(
		create_inbound_entry!(NAME1, i32),
		create_inoutbound_entry!(NAME2, i32),
		create_outbound_entry!("TEST3", i32)
	);
	let x = portlist.find("TEST1").unwrap();
	let x = portlist.find("TEST2").unwrap();
	let x = portlist.find("TEST3").unwrap();

	let portlist = create_port_array!(
		create_inbound_entry!("in", i32, 42),
		create_inoutbound_entry!("inout", i32, 42),
		create_outbound_entry!("out", i32, 42)
	);
	let x = portlist.find("in").unwrap();
	assert_eq!(x.get().unwrap(), Some(42));
	let x = portlist.find("inout").unwrap();
	assert_eq!(x.get().unwrap(), Some(42));

	let portlist = create_port_array!(
		create_inbound_entry!("in", 42),
		create_inoutbound_entry!("inout", 42),
		create_outbound_entry!("out", 42)
	);
	let x = portlist.find("in").unwrap();
	assert_eq!(x.get().unwrap(), Some(42));
	let x = portlist.find("inout").unwrap();
	assert_eq!(x.get().unwrap(), Some(42));

	let portlist = create_port_array!(
		create_inbound_entry!("in", 2 * 42),
		create_inoutbound_entry!("inout", 2 * 42),
		create_outbound_entry!("out", 2 * 42)
	);
	let x = portlist.find("in").unwrap();
	assert_eq!(x.get().unwrap(), Some(84));
	let x = portlist.find("inout").unwrap();
	assert_eq!(x.get().unwrap(), Some(84));

	let portlist = create_port_array!(
		create_inbound_entry!("in", i32, 2 * 42),
		create_inoutbound_entry!("inout", i32, 2 * 42),
		create_outbound_entry!("out", i32, 2 * 42)
	);

	let x = portlist.find("in").unwrap();
	assert_eq!(x.get().unwrap(), Some(84));
	let x = portlist.find("inout").unwrap();
	assert_eq!(x.get().unwrap(), Some(84));

	let _portlist = create_port_array!(
		create_inbound_entry!("in", Test),
		create_inoutbound_entry!("inout", Test),
		create_outbound_entry!("out", Test)
	);

	// This does not work due to parameter ambiguity
	//let val1 = Test::new();
	//let val2 = Test::new();
	//let val3 = Test::new();
	//let _portlist = create_port_array!(create_inbound_entry!("in", val1), create_inoutbound_entry!("inout", val2), create_outbound_entry!("out", val3));

	let portlist = create_port_array!(
		create_inbound_entry!("in", Test, Test::new()),
		create_inoutbound_entry!(
			"inout",
			Test,
			Test {
				f1: 42,
				f2: 3.22,
				f3: vec![String::from("test")]
			}
		),
		create_outbound_entry!(
			"out",
			Test,
			Test {
				f1: 42,
				f2: 3.22,
				f3: vec![String::from("test")]
			}
		)
	);

	let x: Test = portlist
		.find("in")
		.unwrap()
		.get()
		.unwrap()
		.unwrap();
	assert_eq!(x.f1, 42);
	assert_eq!(x.f2, 3.22);
	assert_eq!(x.f3, vec![String::from("test")]);
	let x: Test = portlist
		.find("inout")
		.unwrap()
		.get()
		.unwrap()
		.unwrap();
	assert_eq!(x.f1, 42);
	assert_eq!(x.f2, 3.22);
	assert_eq!(x.f3, vec![String::from("test")]);
}
