// Copyright © 2025 Stephan Kunz
//! Test port binding.

use std::f64::consts::PI;

use dataport::*;

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

macro_rules! test_binding {
	($tp:ty, $name:expr, $value:expr) => {{
		let ip = InBoundPort::<$tp>::new($name);
		assert_eq!(ip.sequence_number(), 0);
		assert!(ip.read().is_err());
		assert!(ip.get().is_none());
		assert_eq!(ip.sequence_number(), 0);
	}
	{
		let op = OutBoundPort::<$tp>::with_value("test", $value);
		let mut ip = InBoundPort::<$tp>::new($name);
		assert!(ip.bind_to_out_port(&op).is_ok());
		assert_eq!(ip.sequence_number(), 1);
		assert_eq!(*ip.read().unwrap(), $value);
		assert_eq!(ip.get().unwrap(), $value);
		assert_eq!(ip.sequence_number(), 1);
	}
	{
		let iop = InOutBoundPort::<$tp>::with_value("test", $value);
		let mut ip = InBoundPort::<$tp>::new($name);
		assert!(ip.bind_to_in_out_port(&iop).is_ok());
		assert_eq!(ip.sequence_number(), 1);
		assert_eq!(*ip.read().unwrap(), $value);
		assert_eq!(ip.get().unwrap(), $value);
		assert_eq!(ip.sequence_number(), 1);
	}
	{
		let iop1 = InOutBoundPort::<$tp>::with_value("test", $value);
		let mut iop2 = InOutBoundPort::<$tp>::new($name);
		assert!(iop2.bind_to_in_out_port(&iop1).is_ok());
		assert_eq!(iop2.sequence_number(), 1);
		assert_eq!(*iop2.read().unwrap(), $value);
		assert_eq!(iop2.get().unwrap(), $value);
		assert_eq!(iop2.sequence_number(), 1);
	}};
}

#[test]
fn binding() {
	#[derive(Clone, Debug, Default, PartialEq)]
	struct MyStruct {
		_f1: i32,
		_f2: f64,
		_f3: String,
		_f4: Vec<f64>,
	}

	let p4_name = String::from("{p4}");
	test_binding!(i32, "p1", 42);
	test_binding!(f64, CONST_NAME, PI);
	test_binding!(String, STATIC_NAME, String::from("hello world"));
	test_binding!(MyStruct, p4_name.as_str(), MyStruct::default());
	test_binding!(Vec<i32>, p4_name.as_str(), vec![1, 2, 3]);
}
