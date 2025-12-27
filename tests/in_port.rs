// Copyright © 2025 Stephan Kunz
//! Test [`InPort`] features.

use std::f64::consts::PI;

use dataport::*;

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

macro_rules! test_getter {
	($tp:ty, $name:expr, $value:expr) => {
		let ip = InputPort::<$tp>::new($name);
		assert_eq!(ip.sequence_number(), 0);
		assert!(ip.read().is_err());
		assert!(ip.get().is_none());
		assert!(ip.take().is_none());
		assert_eq!(ip.sequence_number(), 0);

		let op = OutputPort::<$tp>::with_value("test", $value);
		let ip = InputPort::<$tp>::with_src($name, op);
		assert_eq!(ip.sequence_number(), 1);
		assert_eq!(*ip.read().unwrap(), $value);
		assert_eq!(ip.get().unwrap(), $value);
		assert_eq!(ip.take().unwrap(), $value);
		assert_eq!(ip.sequence_number(), 2);
	};
}

#[test]
fn getter() {
	#[derive(Clone, Debug, Default, PartialEq)]
	struct MyStruct {
		_f1: i32,
		_f2: f64,
		_f3: String,
		_f4: Vec<f64>,
	}

	let p4_name = String::from("p4");
	test_getter!(i32, "p1", 42);
	test_getter!(f64, CONST_NAME, PI);
	test_getter!(String, STATIC_NAME, String::from("hello world"));
	test_getter!(MyStruct, p4_name.as_str(), MyStruct::default());
	test_getter!(Vec<i32>, p4_name.as_str(), vec![1, 2, 3]);
}
