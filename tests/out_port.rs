// Copyright © 2025 Stephan Kunz
//! Test [`OutPort`] features.

use std::f64::consts::PI;

use dataport::{OutPort, PortSetter};

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

macro_rules! test_setter {
	($tp:ty, $name:expr, $value:expr, $value2:expr) => {
		// separate creation and value setting
		let op = OutPort::<$tp>::new($name);
		assert!(op.as_mut().is_err());
		op.set($value);
		assert_eq!(op.replace($value).unwrap(), $value);
		*op.as_mut().unwrap() = $value2;
		assert_eq!(op.replace($value).unwrap(), $value2);
		// separate creation and value replacing
		let op = OutPort::<$tp>::new($name);
		assert!(op.as_mut().is_err());
		assert!(op.replace($value).is_none());
		assert_eq!(op.replace($value2).unwrap(), $value);
		*op.as_mut().unwrap() = $value;
		assert_eq!(op.replace($value2).unwrap(), $value);
		// creation with value
		let op = OutPort::<$tp>::with($name, $value);
		let mut guard = op.as_mut().unwrap();
		assert_eq!(*guard, $value);
		*guard = $value2;
		assert_eq!(*guard, $value2);
		drop(guard);
		assert_eq!(op.replace($value).unwrap(), $value2);
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

	// @TODO: make use of &String possible.
	let p4_name = "p4"; //String::from("p4");
	test_setter!(i32, "p1", 42, 24);
	test_setter!(f64, CONST_NAME, PI, 3.0);
	test_setter!(String, STATIC_NAME, String::from("the answer"), String::from("hello world"));
	test_setter!(
		MyStruct,
		p4_name,
		MyStruct::default(),
		MyStruct {
			_f1: 1,
			..Default::default()
		}
	);
	test_setter!(Vec<i32>, &p4_name, vec![1, 2, 3], vec![4, 5, 6]);
}
