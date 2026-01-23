// Copyright © 2026 Stephan Kunz
//! Test a concept

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::{AnyPortValue, PortCollection, inbound, inoutbound, outbound, port_array};

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
	let portlist = port_array!(inbound!("in", i32), inoutbound!("inout", i32), outbound!("out", i32));
	let x = portlist.find("in").unwrap();
	let x = portlist.find("inout").unwrap();
	let x = portlist.find("out").unwrap();

	let name1 = "test1";
	let name2 = String::from("test2");

	let portlist = port_array!(inbound!(name1, i32), inoutbound!(name2, i32), outbound!("test3", i32));
	let x = portlist.find("test1").unwrap();
	let x = portlist.find("test2").unwrap();
	let x = portlist.find("test3").unwrap();

	let portlist = port_array!(inbound!(NAME1, i32), inoutbound!(NAME2, i32), outbound!("TEST3", i32));
	let x = portlist.find("TEST1").unwrap();
	let x = portlist.find("TEST2").unwrap();
	let x = portlist.find("TEST3").unwrap();

	let portlist = port_array!(
		inbound!("in", i32, 42),
		inoutbound!("inout", i32, 42),
		outbound!("out", i32, 42)
	);
	let x = portlist.find("in").unwrap();
	assert_eq!(x.get().unwrap(), Some(42));
	let x = portlist.find("inout").unwrap();
	assert_eq!(x.get().unwrap(), Some(42));

	let portlist = port_array!(inbound!("in", 42), inoutbound!("inout", 42), outbound!("out", 42));
	let x = portlist.find("in").unwrap();
	assert_eq!(x.get().unwrap(), Some(42));
	let x = portlist.find("inout").unwrap();
	assert_eq!(x.get().unwrap(), Some(42));

	let portlist = port_array!(inbound!("in", 2 * 42), inoutbound!("inout", 2 * 42), outbound!("out", 2 * 42));
	let x = portlist.find("in").unwrap();
	assert_eq!(x.get().unwrap(), Some(84));
	let x = portlist.find("inout").unwrap();
	assert_eq!(x.get().unwrap(), Some(84));

	let portlist = port_array!(
		inbound!("in", i32, 2 * 42),
		inoutbound!("inout", i32, 2 * 42),
		outbound!("out", i32, 2 * 42)
	);

	let x = portlist.find("in").unwrap();
	assert_eq!(x.get().unwrap(), Some(84));
	let x = portlist.find("inout").unwrap();
	assert_eq!(x.get().unwrap(), Some(84));

	let _portlist = port_array!(inbound!("in", Test), inoutbound!("inout", Test), outbound!("out", Test));

	//let val1 = Test::new();
	//let val2 = Test::new();
	//let val3 = Test::new();
	//let _portlist = port_array!(inbound!("in", val1), inoutbound!("inout", val2), outbound!("out", val3));

	let portlist = port_array!(
		inbound!("in", Test, Test::new()),
		inoutbound!(
			"inout",
			Test,
			Test {
				f1: 42,
				f2: 3.22,
				f3: vec![String::from("test")]
			}
		),
		outbound!(
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
