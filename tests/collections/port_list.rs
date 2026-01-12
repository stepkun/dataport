// Copyright © 2026 Stephan Kunz
//! Test [`PortList`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use core::f64::consts::PI;

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, DynamicPortProvider, PortAccessors, PortList, PortProvider, PortVariant,
};

macro_rules! test_creation {
	($tp:ty, $value: expr) => {{
		let mut list = PortList::new();
		assert!(
			list.insert("inbound0", PortVariant::InBound(BoundInPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			list.insert("outbound0", PortVariant::OutBound(BoundOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			list.insert("inoutbound0", PortVariant::InOutBound(BoundInOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			list.insert("inbound1", PortVariant::InBound(BoundInPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.insert("outbound1", PortVariant::OutBound(BoundOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.insert("inoutbound1", PortVariant::InOutBound(BoundInOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			list.insert("inbound2", PortVariant::create_inbound($value))
				.is_ok()
		);
		assert!(
			list.insert("outbound2", PortVariant::create_outbound($value))
				.is_ok()
		);
		assert!(
			list.insert("inoutbound2", PortVariant::create_inoutbound($value))
				.is_ok()
		);

		assert!(list.find("inbound").is_none());
		assert!(list.find("outbound").is_none());
		assert!(list.find_mut("inoutbound").is_none());

		assert!(list.find("inbound0").is_some());
		assert!(list.find("inbound1").is_some());
		assert!(list.find("inbound2").is_some());

		assert!(list.find("outbound0").is_some());
		assert!(list.find("outbound1").is_some());
		assert!(list.find("outbound2").is_some());

		assert!(list.find_mut("inoutbound0").is_some());
		assert!(list.find_mut("inoutbound1").is_some());
		assert!(list.find_mut("inoutbound2").is_some());
	}};
}

#[test]
fn list_creation() {
	test_creation!(bool, true);
	test_creation!(i32, 42);
	test_creation!(f64, PI);
	test_creation!(&str, "str");
	test_creation!(String, String::from("string"));
	test_creation!(Vec<i32>, vec![1, 2, 3]);
	test_creation!(Vec<&str>, vec!["1", "2", "3"]);
	test_creation!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	test_creation!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}

macro_rules! test_accessors {
	($tp:ty, $value1: expr, $value2: expr) => {
		let mut list = PortList::new();
		assert!(
			list.insert("inbound0", PortVariant::InBound(BoundInPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			list.insert("outbound0", PortVariant::OutBound(BoundOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			list.insert("inoutbound0", PortVariant::InOutBound(BoundInOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			list.insert("inbound1", PortVariant::InBound(BoundInPort::with_value($value1)))
				.is_ok()
		);
		assert!(
			list.insert("outbound1", PortVariant::OutBound(BoundOutPort::with_value($value1)))
				.is_ok()
		);
		assert!(
			list.insert(
				"inoutbound1",
				PortVariant::InOutBound(BoundInOutPort::with_value($value1))
			)
			.is_ok()
		);

		assert!(list.get::<$tp>("test").is_err());
		assert_eq!(list.get::<$tp>("inbound0").unwrap(), None);
		assert!(list.get::<$tp>("outbound0").is_err());
		assert_eq!(list.get::<$tp>("inoutbound0").unwrap(), None);
		assert_eq!(list.get::<$tp>("inbound1").unwrap(), Some($value1));
		assert!(list.get::<$tp>("outbound1").is_err());
		assert_eq!(list.get::<$tp>("inoutbound1").unwrap(), Some($value1));

		assert!(list.read::<$tp>("test").is_err());
		assert!(list.read::<$tp>("inbound0").is_err());
		assert!(list.read::<$tp>("inoutbound0").is_err());
		assert!(list.read::<$tp>("outbound0").is_err());
		assert_eq!(*list.read::<$tp>("inbound1").unwrap(), $value1);
		assert!(list.read::<$tp>("outbound1").is_err());
		assert_eq!(*list.read::<$tp>("inoutbound1").unwrap(), $value1);

		assert!(list.try_read::<$tp>("test").is_err());
		assert!(list.try_read::<$tp>("inbound0").is_err());
		assert!(list.try_read::<$tp>("inoutbound0").is_err());
		assert!(list.try_read::<$tp>("outbound0").is_err());
		assert_eq!(*list.try_read::<$tp>("inbound1").unwrap(), $value1);
		assert!(list.try_read::<$tp>("outbound1").is_err());
		assert_eq!(*list.try_read::<$tp>("inoutbound1").unwrap(), $value1);

		assert!(list.set("test", $value2).is_err());
		assert!(list.set("inbound0", $value2).is_err());
		assert!(list.set("outbound0", $value2).is_ok());
		assert!(list.set("inoutbound0", $value2).is_ok());
		assert_eq!(*list.read::<$tp>("inoutbound0").unwrap(), $value2);
		assert!(list.set("inbound1", $value2).is_err());
		assert!(list.set("outbound1", $value2).is_ok());
		assert!(list.set("inoutbound1", $value2).is_ok());
		assert_eq!(list.get::<$tp>("inoutbound1").unwrap(), Some($value2));

		{
			assert!(list.write::<$tp>("test").is_err());
			assert!(list.write::<$tp>("inbound0").is_err());
			let mut g_out = list.write::<$tp>("outbound0").unwrap();
			assert_eq!(*g_out, $value2);
			*g_out = $value1;
			assert_eq!(*g_out, $value1);
			let mut g_inout = list.write::<$tp>("inoutbound0").unwrap();
			assert_eq!(*g_inout, $value2);
			*g_inout = $value1;
			assert_eq!(*g_inout, $value1);
			assert!(list.write::<$tp>("inbound1").is_err());
			let mut g_out = list.write::<$tp>("outbound1").unwrap();
			assert_eq!(*g_out, $value2);
			*g_out = $value1;
			assert_eq!(*g_out, $value1);
			let mut g_inout = list.write::<$tp>("inoutbound1").unwrap();
			assert_eq!(*g_inout, $value2);
			*g_inout = $value1;
			assert_eq!(*g_inout, $value1);
		}
		{
			assert!(list.try_write::<$tp>("test").is_err());
			assert!(list.try_write::<$tp>("inbound0").is_err());
			let mut g_out = list.try_write::<$tp>("outbound0").unwrap();
			assert_eq!(*g_out, $value1);
			*g_out = $value2;
			assert_eq!(*g_out, $value2);
			let mut g_inout = list.try_write::<$tp>("inoutbound0").unwrap();
			assert_eq!(*g_inout, $value1);
			*g_inout = $value2;
			assert_eq!(*g_inout, $value2);
			assert!(list.try_write::<$tp>("inbound1").is_err());
			let mut g_out = list.try_write::<$tp>("outbound1").unwrap();
			assert_eq!(*g_out, $value1);
			*g_out = $value2;
			assert_eq!(*g_out, $value2);
			let mut g_inout = list.try_write::<$tp>("inoutbound1").unwrap();
			assert_eq!(*g_inout, $value1);
			*g_inout = $value2;
			assert_eq!(*g_inout, $value2);
		}
		{
			assert!(list.replace::<$tp>("test", $value1).is_err());
			assert!(list.replace::<$tp>("inbound0", $value1).is_err());
			assert!(list.replace::<$tp>("outbound0", $value1).is_err());
			assert_eq!(
				list.replace::<$tp>("inoutbound0", $value1)
					.unwrap(),
				Some($value2)
			);
			assert!(list.replace::<$tp>("inbound1", $value1).is_err());
			assert!(list.replace::<$tp>("outbound1", $value1).is_err());
			assert_eq!(
				list.replace::<$tp>("inoutbound1", $value1)
					.unwrap(),
				Some($value2)
			);
		}
		{
			assert!(list.take::<$tp>("test").is_err());
			assert!(list.take::<$tp>("inbound0").is_err());
			assert!(list.take::<$tp>("outbound0").is_err());
			assert_eq!(list.take::<$tp>("inoutbound0").unwrap(), Some($value1));
			assert_eq!(list.take::<$tp>("inoutbound0").unwrap(), None);
			assert!(list.take::<$tp>("inbound1").is_err());
			assert!(list.take::<$tp>("outbound1").is_err());
			assert_eq!(list.take::<$tp>("inoutbound1").unwrap(), Some($value1));
			assert_eq!(list.take::<$tp>("inoutbound1").unwrap(), None);
		}
	};
}

#[test]
fn list_accessors() {
	test_accessors!(bool, true, false);
	test_accessors!(i32, 42, 24);
	test_accessors!(f64, PI, 6.0);
	test_accessors!(&str, "str", "other");
	test_accessors!(String, String::from("string"), String::from("other"));
	test_accessors!(Vec<i32>, vec![1, 2, 3], vec![3, 2, 1]);
	test_accessors!(Vec<&str>, vec!["1", "2", "3"], vec!["3", "2", "1"]);
	test_accessors!(
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
	test_accessors!(
		Vec<Vec<f64>>,
		vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
		vec![vec![6.0, 5.0, 4.0], vec![3.0, 2.0, 1.0]]
	);
}
