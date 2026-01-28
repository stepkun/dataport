// Copyright © 2026 Stephan Kunz
//! Test [`BoundInOutPort`]s

#![allow(missing_docs)]

use std::f64::consts::PI;

use dataport::{BindIn, BindInOut, BindOut, BoundInOutPort, PortValueReadGuard, PortValueWriteGuard};

macro_rules! test_get {
	($value:expr) => {
		let iop = BoundInOutPort::with_value($value);
		assert_eq!(iop.get().unwrap(), Some($value));
	};
}

macro_rules! test_read {
	($tp:ty, $value:expr) => {
		let iop = BoundInOutPort::with_value($value);
		let guard: PortValueReadGuard<$tp> = iop.read().unwrap();
		assert_eq!(*guard, $value);
	};
}

macro_rules! test_try_read {
	($tp:ty, $value:expr) => {
		let iop = BoundInOutPort::with_value($value);
		let guard: PortValueReadGuard<$tp> = iop.try_read().unwrap();
		assert_eq!(*guard, $value);
	};
}

macro_rules! test_set {
	($tp:ty, $value:expr) => {
		let mut iop = BoundInOutPort::with_value($value);
		assert!(iop.set($value).is_ok());
	};
}

macro_rules! test_write {
	($tp:ty, $value:expr) => {
		let mut iop = BoundInOutPort::with_value($value);
		let mut guard: PortValueWriteGuard<$tp> = iop.write().unwrap();
		assert_eq!(*guard = $value, ());
	};
}

macro_rules! test_try_write {
	($tp:ty, $value:expr) => {
		let mut iop = BoundInOutPort::with_value($value);
		let mut guard: PortValueWriteGuard<$tp> = iop.try_write().unwrap();
		assert_eq!(*guard = $value, ());
	};
}

macro_rules! test_take_replace {
	($tp:ty, $value:expr, $value2:expr) => {
		let mut iop = BoundInOutPort::with_value($value);
		assert_eq!(iop.replace($value2).unwrap(), Some($value));
		assert_eq!(iop.take().unwrap(), Some($value2));
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

	test_take_replace!(bool, true, false);
	test_take_replace!(i32, 42, 24);
	test_take_replace!(f64, PI, PI / 2.0);
	test_take_replace!(&str, "str", "");
	test_take_replace!(String, String::from("string"), String::from(""));
	test_take_replace!(Vec<i32>, vec![1, 2, 3], vec![3, 2, 1]);
	test_take_replace!(Vec<&str>, vec!["1", "2", "3"], vec!["3", "2", "1"]);
	test_take_replace!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		],
		vec![
			String::from("3"),
			String::from("2"),
			String::from("1")
		]
	);
	test_take_replace!(
		Vec<Vec<f64>>,
		vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
		vec![vec![4.0, 5.0, 6.0], vec![1.0, 2.0, 3.0]]
	);
}
