// Copyright © 2026 Stephan Kunz
//! Test binding of bind type ports

#![allow(missing_docs)]

use std::f64::consts::PI;

use dataport::{BoundInOutPort, BoundInPort, BoundOutPort, PortVariant};

macro_rules! test_binding {
	($tp:ty, $value:expr) => {
		let mut op = PortVariant::OutBound(BoundOutPort::new::<$tp>());
		let mut iop = PortVariant::InOutBound(BoundInOutPort::new::<$tp>());
		let mut ip = PortVariant::InBound(BoundInPort::new::<$tp>());

		assert!(iop.use_from_variant(&op).is_ok());
		assert!(op.use_from_variant(&iop).is_ok());
		assert!(op.use_from_variant(&ip).is_ok());
		assert!(iop.use_from_variant(&ip).is_ok());
		assert!(ip.use_from_variant(&iop).is_ok());
		assert!(ip.use_from_variant(&op).is_ok());

		assert!(op.set($value).is_ok());
		assert_eq!(iop.get().unwrap(), Some($value));
		assert_eq!(ip.get().unwrap(), Some($value));
	};
}
#[test]
fn binding() {
	test_binding!(bool, true);
	test_binding!(i32, 42);
	test_binding!(f64, PI);
	test_binding!(&str, "str");
	test_binding!(String, String::from("string"));
	test_binding!(Vec<i32>, vec![1, 2, 3]);
	test_binding!(Vec<&str>, vec!["1", "2", "3"]);
	test_binding!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	test_binding!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}
