// Copyright © 2025 Stephan Kunz
//! Test [`InOutPort`] features.

use std::f64::consts::PI;

use dataport::{InOutPort, PortGetter, PortSetter};

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

macro_rules! test_getter_setter {
	($tp:ty, $name:expr, $value:expr, $value2:expr) => {
		// creation without value
		let iop = InOutPort::<$tp>::new($name);
		// connect input and output!
		assert!(iop.set_src(iop.dest()).is_none());
		assert!(iop.read().is_err());
		assert!(iop.write().is_err());
		assert!(iop.get().is_none());
		iop.set($value);
		assert_eq!(iop.replace($value2).unwrap(), $value);
		iop.propagate();
		assert_eq!(iop.get().unwrap(), $value2);
		assert_eq!(*iop.read().unwrap(), $value2);
		assert_eq!(iop.take().unwrap(), $value2);
		assert!(iop.get().is_none());
		// creation with value
		let iop = InOutPort::<$tp>::with_value($name, $value);
		// connect input and output!
		assert!(iop.set_src(iop.dest()).is_none());
		iop.set($value);
		let mut guard = iop.write().unwrap();
		assert_eq!(*guard, $value);
		*guard = $value2;
		assert_eq!(*guard, $value2);
		drop(guard);
		assert_eq!(iop.replace($value).unwrap(), $value2);
		assert_eq!(iop.take().unwrap(), $value);
		assert!(iop.replace($value2).is_none());
		iop.propagate();
		assert_eq!(iop.get().unwrap(), $value2);
		assert_eq!(*iop.read().unwrap(), $value2);
		assert_eq!(iop.take().unwrap(), $value2);
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

	let p4_name = String::from("p4");
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
