// Copyright © 2025 Stephan Kunz
//! Test [`InPort`] features.

use dataport::{InPort, PortGetter};

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

macro_rules! test_getter {
	($tp:ty, $name:expr, $value:expr) => {
		let ip = InPort::<$tp>::new($name);
		assert!(ip.as_ref().is_err());
		assert!(ip.get().is_none());
		assert!(ip.take().is_none());
	};
}

#[test]
fn getter() {
	#[derive(Clone, Default)]
	struct MyStruct {
		_f1: i32,
		_f2: f64,
		_f3: String,
		_f4: Vec<f64>,
	}

	// @TODO: make use of &String possible.
	let p4_name = "p4"; //String::from("p4");
	test_getter!(i32, "p1", 42);
	test_getter!(f64, CONST_NAME, PI);
	test_getter!(String, STATIC_NAME, String::from("hello world"));
	test_getter!(MyStruct, p4_name, MyStruct::default());
	test_getter!(Vec<i32>, &p4_name, vec![1, 2, 3]);
}
