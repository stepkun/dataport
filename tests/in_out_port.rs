// Copyright © 2025 Stephan Kunz
//! Test [`InOutPort`] features.

use std::f64::consts::PI;

use dataport::*;

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

macro_rules! test_getter_setter {
	($tp:ty, $name:expr, $value:expr, $value2:expr) => {
		// creation without value
		let iop = InOutBoundPort::<$tp>::new($name);
		assert!(iop.read().is_err());
		assert!(iop.write().is_err());
		assert!(iop.get().is_none());
		assert_eq!(iop.sequence_number(), 0);
		iop.set($value);
		assert_eq!(iop.sequence_number(), 1);
		assert_eq!(iop.replace($value2).unwrap(), $value);
		assert_eq!(iop.sequence_number(), 2);
		assert_eq!(iop.get().unwrap(), $value2);
		assert_eq!(*iop.read().unwrap(), $value2);
		assert_eq!(iop.take().unwrap(), $value2);
		assert_eq!(iop.sequence_number(), 3);
		assert!(iop.get().is_none());
		// creation with value
		let iop = InOutBoundPort::<$tp>::with_value($name, $value);
		assert_eq!(iop.sequence_number(), 1);
		iop.set($value);
		assert_eq!(iop.sequence_number(), 2);
		let mut guard = iop.write().unwrap();
		assert_eq!(*guard, $value);
		*guard = $value2;
		assert_eq!(*guard, $value2);
		drop(guard);
		assert_eq!(iop.sequence_number(), 3);
		assert_eq!(iop.replace($value).unwrap(), $value2);
		assert_eq!(iop.sequence_number(), 4);
		assert_eq!(iop.take().unwrap(), $value);
		assert_eq!(iop.sequence_number(), 5);
		assert!(iop.replace($value2).is_none());
		assert_eq!(iop.sequence_number(), 6);
		assert_eq!(iop.get().unwrap(), $value2);
		assert_eq!(*iop.read().unwrap(), $value2);
		assert_eq!(iop.take().unwrap(), $value2);
		assert_eq!(iop.sequence_number(), 7);
		assert!(iop.get().is_none());
	};
}

#[test]
fn getter_setter() {
	#[derive(Clone, Debug, Default, PartialEq)]
	struct MyStruct {
		_f1: i32,
		_f2: f64,
		_f3: String,
		_f4: Vec<f64>,
	}

	let p4_name = String::from("{p4}");
	test_getter_setter!(i32, "p1", 42, 24);
	test_getter_setter!(f64, CONST_NAME, PI, 3.0);
	test_getter_setter!(String, STATIC_NAME, String::from("the answer"), String::from("hello world"));
	test_getter_setter!(
		MyStruct,
		p4_name.as_str(),
		MyStruct::default(),
		MyStruct {
			_f1: 1,
			..Default::default()
		}
	);
	test_getter_setter!(Vec<i32>, p4_name.as_str(), vec![1, 2, 3], vec![4, 5, 6]);
}
