// Copyright © 2026 Stephan Kunz
//! Test [`Portmap`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use core::f64::consts::PI;

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, DynamicPortProvider, PortAccessors, PortMap, PortProvider, PortVariant,
};

macro_rules! test_creation {
	($tp:ty, $value: expr) => {{
		let mut map = PortMap::new();
		assert!(
			map.insert("inbound0", PortVariant::InBound(BoundInPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			map.insert("outbound0", PortVariant::OutBound(BoundOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			map.insert("inoutbound0", PortVariant::InOutBound(BoundInOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			map.insert("inbound1", PortVariant::InBound(BoundInPort::with_value($value)))
				.is_ok()
		);
		assert!(
			map.insert("outbound1", PortVariant::OutBound(BoundOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			map.insert("inoutbound1", PortVariant::InOutBound(BoundInOutPort::with_value($value)))
				.is_ok()
		);
		assert!(
			map.insert("inbound2", PortVariant::create_inbound($value))
				.is_ok()
		);
		assert!(
			map.insert("outbound2", PortVariant::create_outbound($value))
				.is_ok()
		);
		assert!(
			map.insert("inoutbound2", PortVariant::create_inoutbound($value))
				.is_ok()
		);

		assert!(map.find("inbound").is_none());
		assert!(map.find("outbound").is_none());
		assert!(map.find_mut("inoutbound").is_none());

		assert!(map.find("inbound0").is_some());
		assert!(map.find("inbound1").is_some());
		assert!(map.find("inbound2").is_some());

		assert!(map.find("outbound0").is_some());
		assert!(map.find("outbound1").is_some());
		assert!(map.find("outbound2").is_some());

		assert!(map.find_mut("inoutbound0").is_some());
		assert!(map.find_mut("inoutbound1").is_some());
		assert!(map.find_mut("inoutbound2").is_some());
	}};
}

#[test]
fn map_creation() {
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
		let mut map = PortMap::new();
		assert!(
			map.insert("inbound0", PortVariant::InBound(BoundInPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			map.insert("outbound0", PortVariant::OutBound(BoundOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			map.insert("inoutbound0", PortVariant::InOutBound(BoundInOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			map.insert("inbound1", PortVariant::InBound(BoundInPort::with_value($value1)))
				.is_ok()
		);
		assert!(
			map.insert("outbound1", PortVariant::OutBound(BoundOutPort::with_value($value1)))
				.is_ok()
		);
		assert!(
			map.insert(
				"inoutbound1",
				PortVariant::InOutBound(BoundInOutPort::with_value($value1))
			)
			.is_ok()
		);

		assert!(map.get::<$tp>("test").is_err());
		assert_eq!(map.get::<$tp>("inbound0").unwrap(), None);
		assert!(map.get::<$tp>("outbound0").is_err());
		assert_eq!(map.get::<$tp>("inoutbound0").unwrap(), None);
		assert_eq!(map.get::<$tp>("inbound1").unwrap(), Some($value1));
		assert!(map.get::<$tp>("outbound1").is_err());
		assert_eq!(map.get::<$tp>("inoutbound1").unwrap(), Some($value1));

		assert!(map.read::<$tp>("test").is_err());
		assert!(map.read::<$tp>("inbound0").is_err());
		assert!(map.read::<$tp>("inoutbound0").is_err());
		assert!(map.read::<$tp>("outbound0").is_err());
		assert_eq!(*map.read::<$tp>("inbound1").unwrap(), $value1);
		assert!(map.read::<$tp>("outbound1").is_err());
		assert_eq!(*map.read::<$tp>("inoutbound1").unwrap(), $value1);

		assert!(map.try_read::<$tp>("test").is_err());
		assert!(map.try_read::<$tp>("inbound0").is_err());
		assert!(map.try_read::<$tp>("inoutbound0").is_err());
		assert!(map.try_read::<$tp>("outbound0").is_err());
		assert_eq!(*map.try_read::<$tp>("inbound1").unwrap(), $value1);
		assert!(map.try_read::<$tp>("outbound1").is_err());
		assert_eq!(*map.try_read::<$tp>("inoutbound1").unwrap(), $value1);

		assert!(map.set("test", $value2).is_err());
		assert!(map.set("inbound0", $value2).is_err());
		assert!(map.set("outbound0", $value2).is_ok());
		assert!(map.set("inoutbound0", $value2).is_ok());
		assert_eq!(*map.read::<$tp>("inoutbound0").unwrap(), $value2);
		assert!(map.set("inbound1", $value2).is_err());
		assert!(map.set("outbound1", $value2).is_ok());
		assert!(map.set("inoutbound1", $value2).is_ok());
		assert_eq!(map.get::<$tp>("inoutbound1").unwrap(), Some($value2));

		{
			assert!(map.write::<$tp>("test").is_err());
			assert!(map.write::<$tp>("inbound0").is_err());
			let mut g_out = map.write::<$tp>("outbound0").unwrap();
			assert_eq!(*g_out, $value2);
			*g_out = $value1;
			assert_eq!(*g_out, $value1);
			let mut g_inout = map.write::<$tp>("inoutbound0").unwrap();
			assert_eq!(*g_inout, $value2);
			*g_inout = $value1;
			assert_eq!(*g_inout, $value1);
			assert!(map.write::<$tp>("inbound1").is_err());
			let mut g_out = map.write::<$tp>("outbound1").unwrap();
			assert_eq!(*g_out, $value2);
			*g_out = $value1;
			assert_eq!(*g_out, $value1);
			let mut g_inout = map.write::<$tp>("inoutbound1").unwrap();
			assert_eq!(*g_inout, $value2);
			*g_inout = $value1;
			assert_eq!(*g_inout, $value1);
		}
		{
			assert!(map.try_write::<$tp>("test").is_err());
			assert!(map.try_write::<$tp>("inbound0").is_err());
			let mut g_out = map.try_write::<$tp>("outbound0").unwrap();
			assert_eq!(*g_out, $value1);
			*g_out = $value2;
			assert_eq!(*g_out, $value2);
			let mut g_inout = map.try_write::<$tp>("inoutbound0").unwrap();
			assert_eq!(*g_inout, $value1);
			*g_inout = $value2;
			assert_eq!(*g_inout, $value2);
			assert!(map.try_write::<$tp>("inbound1").is_err());
			let mut g_out = map.try_write::<$tp>("outbound1").unwrap();
			assert_eq!(*g_out, $value1);
			*g_out = $value2;
			assert_eq!(*g_out, $value2);
			let mut g_inout = map.try_write::<$tp>("inoutbound1").unwrap();
			assert_eq!(*g_inout, $value1);
			*g_inout = $value2;
			assert_eq!(*g_inout, $value2);
		}
		{
			assert!(map.replace::<$tp>("test", $value1).is_err());
			assert!(map.replace::<$tp>("inbound0", $value1).is_err());
			assert!(map.replace::<$tp>("outbound0", $value1).is_err());
			assert_eq!(
				map.replace::<$tp>("inoutbound0", $value1)
					.unwrap(),
				Some($value2)
			);
			assert!(map.replace::<$tp>("inbound1", $value1).is_err());
			assert!(map.replace::<$tp>("outbound1", $value1).is_err());
			assert_eq!(
				map.replace::<$tp>("inoutbound1", $value1)
					.unwrap(),
				Some($value2)
			);
		}
		{
			assert!(map.take::<$tp>("test").is_err());
			assert!(map.take::<$tp>("inbound0").is_err());
			assert!(map.take::<$tp>("outbound0").is_err());
			assert_eq!(map.take::<$tp>("inoutbound0").unwrap(), Some($value1));
			assert_eq!(map.take::<$tp>("inoutbound0").unwrap(), None);
			assert!(map.take::<$tp>("inbound1").is_err());
			assert!(map.take::<$tp>("outbound1").is_err());
			assert_eq!(map.take::<$tp>("inoutbound1").unwrap(), Some($value1));
			assert_eq!(map.take::<$tp>("inoutbound1").unwrap(), None);
		}
	};
}

#[test]
fn map_accessors() {
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
