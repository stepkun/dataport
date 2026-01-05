// Copyright © 2025 Stephan Kunz
//! Test [`OutPort`] features.

#![allow(unused)]

use std::f64::consts::PI;

use dataport::*;

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

macro_rules! test_setter {
	($tp:ty, $name:expr, $value:expr, $value2:expr) => {
		// separate creation and value setting
		let op = OutBoundPort::<$tp>::new($name);
		assert!(op.write().is_err());
		op.set($value);
		*op.write().unwrap() = $value2;
		// creation with value
		let op = OutBoundPort::<$tp>::with_value($name, $value);
		let mut guard = op.write().unwrap();
		assert_eq!(*guard, $value);
		*guard = $value2;
		assert_eq!(*guard, $value2);
		drop(guard);
	};
}

#[test]
fn setter() {
	#[derive(Debug, Default, PartialEq)]
	struct MyStruct {
		_f1: i32,
		_f2: f64,
		_f3: String,
		_f4: Vec<f64>,
	}

	let p4_name = String::from("{p4}");
	test_setter!(i32, "p1", 42, 24);
	test_setter!(f64, CONST_NAME, PI, 3.0);
	test_setter!(String, STATIC_NAME, String::from("the answer"), String::from("hello world"));
	test_setter!(
		MyStruct,
		p4_name.as_str(),
		MyStruct::default(),
		MyStruct {
			_f1: 1,
			..Default::default()
		}
	);
	test_setter!(Vec<i32>, p4_name.as_str(), vec![1, 2, 3], vec![4, 5, 6]);
}
