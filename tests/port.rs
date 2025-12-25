// Copyright © 2025 Stephan Kunz
//! Test [`Port`] and connection features.

use std::f64::consts::PI;

use dataport::*;

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

/// Permutations of the port combinations.
macro_rules! test_equality {
	($tp1:ty, $name1:literal, $tp2:ty, $name2:expr) => {
		// equal port type, equal data type and equal name
		assert_eq!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_in_port::<$tp1>($name1)
		);
		assert_eq!(
			Port::create_in_port::<$tp2>($name2),
			Port::create_in_port::<$tp2>($name2)
		);
		assert_eq!(
			Port::create_inout_port::<$tp1>($name1),
			Port::create_inout_port::<$tp1>($name1)
		);
		assert_eq!(
			Port::create_inout_port::<$tp2>($name2),
			Port::create_inout_port::<$tp2>($name2)
		);
		assert_eq!(
			Port::create_out_port::<$tp1>($name1),
			Port::create_out_port::<$tp1>($name1)
		);
		assert_eq!(
			Port::create_out_port::<$tp2>($name2),
			Port::create_out_port::<$tp2>($name2)
		);
		// equal port type, equal data type and different name
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_in_port::<$tp1>($name2)
		);
		assert_ne!(
			Port::create_inout_port::<$tp1>($name1),
			Port::create_inout_port::<$tp1>($name2)
		);
		assert_ne!(
			Port::create_out_port::<$tp1>($name1),
			Port::create_out_port::<$tp1>($name2)
		);
		// equal port type, different data type and equal name
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_in_port::<$tp2>($name1)
		);
		// equal port type, different data type and different name
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_in_port::<$tp2>($name2)
		);

		// different port type, equal data type and equal name
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_inout_port::<$tp1>($name1)
		);
		assert_ne!(
			Port::create_in_port::<$tp2>($name2),
			Port::create_inout_port::<$tp2>($name2)
		);
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_out_port::<$tp1>($name1)
		);
		assert_ne!(
			Port::create_in_port::<$tp2>($name2),
			Port::create_out_port::<$tp2>($name2)
		);
		assert_ne!(
			Port::create_inout_port::<$tp1>($name1),
			Port::create_out_port::<$tp1>($name1)
		);
		assert_ne!(
			Port::create_inout_port::<$tp2>($name2),
			Port::create_out_port::<$tp2>($name2)
		);
		// different port type, equal data type and different name
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_inout_port::<$tp1>($name2)
		);
		assert_ne!(
			Port::create_in_port::<$tp2>($name1),
			Port::create_inout_port::<$tp2>($name2)
		);
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_out_port::<$tp1>($name2)
		);
		assert_ne!(
			Port::create_in_port::<$tp2>($name1),
			Port::create_out_port::<$tp2>($name2)
		);
		assert_ne!(
			Port::create_inout_port::<$tp1>($name1),
			Port::create_out_port::<$tp1>($name2)
		);
		assert_ne!(
			Port::create_inout_port::<$tp2>($name1),
			Port::create_out_port::<$tp2>($name2)
		);
		// different port type, different data type and equal name
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_inout_port::<$tp2>($name1)
		);
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_out_port::<$tp2>($name1)
		);
		assert_ne!(
			Port::create_inout_port::<$tp1>($name1),
			Port::create_out_port::<$tp2>($name1)
		);
		// different port type, different data type and different name
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_inout_port::<$tp2>($name2)
		);
		assert_ne!(
			Port::create_in_port::<$tp1>($name1),
			Port::create_out_port::<$tp2>($name2)
		);
		assert_ne!(
			Port::create_inout_port::<$tp1>($name1),
			Port::create_out_port::<$tp2>($name2)
		);
	};
}

#[test]
fn port_equality() {
	struct MyStruct {
		_f1: i32,
		_f2: f64,
		_f3: String,
		_f4: Vec<f64>,
	}

	let p4_name: &str = "p4"; //String::from("p4");
	test_equality!(i32, "p1", f64, "p2");
	test_equality!(String, "p1", &str, CONST_NAME);
	test_equality!(MyStruct, "p1", Vec<i32>, STATIC_NAME);
	test_equality!(Vec<String>, "p1", Vec<Vec<String>>, &p4_name);
}

macro_rules! test_connections {
	($tp:ty, $name: expr, $value:expr) => {
		let ip = InPort::<$tp>::new($name);
		let iop = InOutPort::<$tp>::new($name);
		let op = OutPort::<$tp>::new($name);

		op.set($value);
		assert!(iop.set_src(op).is_none());
		assert!(iop.src().is_some());
		assert_eq!(iop.get().unwrap(), $value);

		iop.propagate();

		assert!(ip.set_src(iop).is_none());
		assert_eq!(ip.get().unwrap(), $value);
	};
}

#[test]
fn port_connections() {
	let p4_name = String::from("p4");
	test_connections!(i32, "p1", 42);
	test_connections!(f64, CONST_NAME, PI);
	test_connections!(String, STATIC_NAME, "hello world");
	test_connections!(Vec<i32>, p4_name.as_str(), vec![1, 2, 3]);
}
