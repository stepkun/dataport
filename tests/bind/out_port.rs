// Copyright © 2026 Stephan Kunz
//! Test [`BoundOutPort`]s

#![allow(missing_docs)]

use std::f64::consts::PI;

use dataport::{BindOut, BoundOutPort, PortValueWriteGuard};

macro_rules! test_set {
	($tp:ty, $value:expr) => {
		let mut op = BoundOutPort::with_value($value);
		assert!(op.set($value).is_ok());
	};
}

macro_rules! test_write {
	($tp:ty, $value:expr) => {
		let mut op = BoundOutPort::with_value($value);
		let mut guard: PortValueWriteGuard<$tp> = op.write().unwrap();
		assert_eq!(*guard = $value, ());
	};
}

macro_rules! test_try_write {
	($tp:ty, $value:expr) => {
		let mut op = BoundOutPort::with_value($value);
		let mut guard: PortValueWriteGuard<$tp> = op.try_write().unwrap();
		assert_eq!(*guard = $value, ());
	};
}

#[test]
fn access() {
	test_set!(bool, true);
	test_set!(i32, 42);
	test_set!(f64, PI);
	test_set!(str, "str");
	test_set!(String, String::from("string"));
	test_set!(Vec<i32>, vec![1, 2, 3]);
	test_set!(Vec<str>, vec!["1", "2", "3"]);
	test_set!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	test_set!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);

	test_write!(bool, true);
	test_write!(i32, 42);
	test_write!(f64, PI);
	test_write!(&str, "str");
	test_write!(String, String::from("string"));
	test_write!(Vec<i32>, vec![1, 2, 3]);
	test_write!(Vec<&str>, vec!["1", "2", "3"]);
	test_write!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	test_write!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);

	test_try_write!(bool, true);
	test_try_write!(i32, 42);
	test_try_write!(f64, PI);
	test_try_write!(&str, "str");
	test_try_write!(String, String::from("string"));
	test_try_write!(Vec<i32>, vec![1, 2, 3]);
	test_try_write!(Vec<&str>, vec!["1", "2", "3"]);
	test_try_write!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	test_try_write!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}
