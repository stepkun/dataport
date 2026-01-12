// Copyright © 2026 Stephan Kunz
//! Test [`BoundInPort`]s

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use std::f64::consts::PI;

use dataport::{BindIn, BoundInPort, PortValueReadGuard};

macro_rules! test_get {
	($value:expr) => {
		let ip = BoundInPort::with_value($value);
		assert_eq!(ip.get().unwrap(), Some($value));
	};
}

macro_rules! test_read {
	($tp:ty, $value:expr) => {
		let ip = BoundInPort::with_value($value);
		let guard: PortValueReadGuard<$tp> = ip.read().unwrap();
		assert_eq!(*guard, $value);
	};
}

macro_rules! test_try_read {
	($tp:ty, $value:expr) => {
		let ip = BoundInPort::with_value($value);
		let guard: PortValueReadGuard<$tp> = ip.try_read().unwrap();
		assert_eq!(*guard, $value);
	};
}

#[test]
fn access() {
	test_get!(true);
	test_get!(42);
	test_get!(PI);
	test_get!("str");
	test_get!(String::from("string"));
	test_get!(vec![1, 2, 3]);
	test_get!(vec!["1", "2", "3"]);
	test_get!(vec![
		String::from("1"),
		String::from("2"),
		String::from("3")
	]);
	test_get!(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);

	test_read!(bool, true);
	test_read!(i32, 42);
	test_read!(f64, PI);
	test_read!(&str, "str");
	test_read!(String, String::from("string"));
	test_read!(Vec<i32>, vec![1, 2, 3]);
	test_read!(Vec<&str>, vec!["1", "2", "3"]);
	test_read!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	test_read!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);

	test_try_read!(bool, true);
	test_try_read!(i32, 42);
	test_try_read!(f64, PI);
	test_try_read!(&str, "str");
	test_try_read!(String, String::from("string"));
	test_try_read!(Vec<i32>, vec![1, 2, 3]);
	test_try_read!(Vec<&str>, vec!["1", "2", "3"]);
	test_try_read!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	test_try_read!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}
