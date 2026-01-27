// Copyright © 2026 Stephan Kunz
//! Test [`PortMap`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use core::f64::consts::PI;

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, Error, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut,
	PortCollectionMut, PortMap, PortVariant, create_inbound_entry, create_inoutbound_entry, create_outbound_entry,
	create_port_map,
};
use std::sync::Arc;

macro_rules! test_creation {
	($tp:ty, $value: expr) => {{
		let mut map = PortMap::default();
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

/// Special test struct
#[derive(Debug)]
struct NoType;

macro_rules! test_accessors {
	($tp:ty, $value1: expr, $value2: expr) => {
		let mut map = PortMap::default();
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

		assert!(!map.contains_name("test"));
		assert!(!map.contains::<$tp>("test").unwrap());
		assert!(map.contains::<NoType>("inbound0").is_err());
		assert!(map.contains_name("inbound0"));
		assert!(map.contains::<$tp>("inbound0").unwrap());
		assert!(map.contains_name("inoutbound0"));
		assert!(map.contains::<$tp>("inoutbound0").unwrap());
		assert!(map.contains_name("outbound0"));
		assert!(map.contains::<$tp>("outbound0").unwrap());

		assert!(map.get::<$tp>("test").is_err());
		assert_eq!(map.get::<$tp>("inbound0").unwrap(), None);
		assert_eq!(map.sequence_number("inbound0"), Ok(0));
		assert!(map.get::<$tp>("outbound0").is_err());
		assert_eq!(map.get::<$tp>("inoutbound0").unwrap(), None);
		assert_eq!(map.sequence_number("inoutbound0"), Ok(0));
		assert_eq!(map.get::<$tp>("inbound1").unwrap(), Some($value1));
		assert_eq!(map.sequence_number("inbound1"), Ok(1));
		assert!(map.get::<$tp>("outbound1").is_err());
		assert_eq!(map.sequence_number("outbound1"), Ok(1));
		assert_eq!(map.get::<$tp>("inoutbound1").unwrap(), Some($value1));
		assert_eq!(map.sequence_number("inoutbound1"), Ok(1));

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

macro_rules! test_connections {
	($tp:ty, $value1: expr, $value2: expr) => {
		let mut map = PortMap::default();
		assert!(
			map.insert("outbound", PortVariant::create_outbound($value1))
				.is_ok()
		);
		assert!(
			map.insert("inbound", PortVariant::InBound(BoundInPort::new::<$tp>()))
				.is_ok()
		);

		let mut map2 = PortMap::default();
		assert!(
			map2.insert("inoutbound", PortVariant::InOutBound(BoundInOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			map2.insert("outbound", PortVariant::create_outbound($value1))
				.is_ok()
		);
		assert!(
			map2.insert("inbound", PortVariant::InBound(BoundInPort::new::<$tp>()))
				.is_ok()
		);

		let mut invalid = PortMap::default();
		assert!(
			invalid
				.insert("invalid", PortVariant::create_inoutbound(NoType))
				.is_ok()
		);

		assert!(
			map.connect_to("notthere", &invalid, "invalid")
				.is_err()
		);
		assert!(
			map.connect_to("inbound", &invalid, "notthere")
				.is_err()
		);
		assert!(
			map.connect_to("inbound", &invalid, "invalid")
				.is_err()
		);
		assert!(
			map2.connect_to("inoutbound", &invalid, "invalid")
				.is_err()
		);
		assert!(
			map.connect_to("outbound", &invalid, "invalid")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("notthere", &map, "inbound")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &map2, "notthere")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &map, "inbound")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &map2, "inoutbound")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &map, "outbound")
				.is_err()
		);

		assert!(
			map2.connect_to("inoutbound", &map, "outbound")
				.is_ok()
		);
		assert!(
			map.connect_to("inbound", &map2, "inoutbound")
				.is_ok()
		);

		assert_eq!(map.get("inbound").unwrap(), Some($value1));

		assert!(map.set("outbound", $value2).is_ok());
		assert_eq!(map.get("inbound").unwrap(), Some($value2));

		// @TODO: is that really ok?
		assert!(
			map.connect_to("inbound", &map2, "inbound")
				.is_ok()
		);
		// @TODO: is that really ok?
		assert!(
			map.connect_to("outbound", &map2, "outbound")
				.is_ok()
		);
	};
}

#[test]
fn map_connection() {
	test_connections!(bool, true, false);
	test_connections!(i32, 42, 24);
	test_connections!(f64, PI, 6.0);
	test_connections!(&str, "str", "other");
	test_connections!(String, String::from("string"), String::from("other"));
	test_connections!(Vec<i32>, vec![1, 2, 3], vec![3, 2, 1]);
	test_connections!(Vec<&str>, vec!["1", "2", "3"], vec!["3", "2", "1"]);
	test_connections!(
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
	test_connections!(
		Vec<Vec<f64>>,
		vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
		vec![vec![6.0, 5.0, 4.0], vec![3.0, 2.0, 1.0]]
	);
}

macro_rules! test_deref {
	($tp:ty, $value: expr) => {
		let mut map = PortMap::default();
		let mut map2 = create_port_map!(create_inbound_entry!("test", $tp, $value));
		map.append(&mut map2);
	};
}

#[test]
fn map_deref() {
	test_deref!(bool, true);
	test_deref!(i32, 42);
	test_deref!(f64, PI);
	test_deref!(&str, "str");
	test_deref!(String, String::from("string"));
	test_deref!(Vec<i32>, vec![1, 2, 3]);
	test_deref!(Vec<&str>, vec!["1", "2", "3"]);
	test_deref!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	test_deref!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}

macro_rules! test_port_collection_mut {
	($tp:ty, $value: expr, $tp2:ty, $value2: expr) => {
		let mut map = create_port_map!(
			create_inbound_entry!("in", $tp, $value),
			create_inoutbound_entry!("inout", $tp, $value),
			create_outbound_entry!("out", $tp, $value),
			create_inoutbound_entry!("empty", $tp),
			create_inbound_entry!("delete1", $tp),
			create_inoutbound_entry!("delete2", $tp, $value),
			create_outbound_entry!("delete3", $tp, $value),
		);

		let entry: (Arc<str>, PortVariant) = create_inbound_entry!("delete1", $tp2, $value2);
		assert_eq!(map.insert(entry.0, entry.1), Err(Error::AlreadyInCollection));
		assert_eq!(map.get::<$tp>("delete1"), Ok(None));

		assert_eq!(map.remove::<$tp>("not_there"), Err(Error::NotFound));

		assert_eq!(map.get::<$tp2>("not_there"), Err(Error::NotFound));
		assert_eq!(map.get::<$tp2>("in"), Err(Error::DataType));
		assert!(map.read::<$tp2>("in").is_err());
		assert!(map.try_read::<$tp2>("in").is_err());
		assert_eq!(map.get::<$tp2>("inout"), Err(Error::DataType));
		assert!(map.read::<$tp2>("inout").is_err());
		assert!(map.try_read::<$tp2>("inout").is_err());
		assert_eq!(map.replace::<$tp2>("inout", $value2), Err(Error::DataType));
		assert_eq!(map.take::<$tp2>("inout"), Err(Error::DataType));
		assert_eq!(map.set::<$tp2>("inout", $value2), Err(Error::DataType));
		assert!(map.write::<$tp2>("inout").is_err());
		assert!(map.try_write::<$tp2>("inout").is_err());
		assert_eq!(map.set::<$tp2>("out", $value2), Err(Error::DataType));
		assert!(map.write::<$tp2>("out").is_err());
		assert!(map.try_write::<$tp2>("out").is_err());

		assert_eq!(map.remove::<$tp2>("delete1"), Err(Error::DataType));
		assert_eq!(map.remove::<$tp>("delete1"), Ok(None));
		assert_eq!(map.remove::<$tp2>("delete2"), Err(Error::DataType));
		assert_eq!(map.remove::<$tp>("delete2"), Ok(Some($value)));
		assert_eq!(map.remove::<$tp2>("delete3"), Err(Error::DataType));
		assert_eq!(map.remove::<$tp>("delete3"), Ok(Some($value)));

		let inout_guard = map.write::<$tp>("inout").unwrap();
		assert!(map.try_read::<$tp>("inout").is_err());
		assert!(map.try_write::<$tp>("inout").is_err());
		assert_eq!(*inout_guard, $value);

		assert!(map.write::<$tp>("empty").is_err());
		assert!(map.try_write::<$tp>("empty").is_err());
	};
}

#[test]
fn map_port_collection_mut() {
	test_port_collection_mut!(bool, true, i32, 42);
	test_port_collection_mut!(i32, 42, f64, PI);
	test_port_collection_mut!(f64, PI, &str, "str");
	test_port_collection_mut!(&str, "str", String, String::from("string"));
	test_port_collection_mut!(String, String::from("string"), Vec<i32>, vec![1, 2, 3]);
	test_port_collection_mut!(Vec<i32>, vec![1, 2, 3], Vec<&str>, vec!["1", "2", "3"]);
	test_port_collection_mut!(Vec<&str>, vec!["1", "2", "3"], bool, true);
	test_port_collection_mut!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		],
		bool,
		true
	);
	test_port_collection_mut!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]], bool, true);
}
