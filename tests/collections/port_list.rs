// Copyright © 2026 Stephan Kunz
//! Test [`PortList`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use core::f64::consts::PI;

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, Error, PortCollection, PortCollectionAccessors,
	PortCollectionAccessorsCommon, PortCollectionAccessorsMut, PortCollectionMut, PortList, PortVariant,
	create_inbound_entry, create_inoutbound_entry, create_outbound_entry, create_port_list,
};

use std::sync::Arc;

macro_rules! test_creation {
	($tp:ty, $value: expr) => {{
		let mut list = PortList::default();
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

/// Special test struct
#[derive(Debug)]
struct NoType;

macro_rules! test_accessors {
	($tp:ty, $value1: expr, $value2: expr) => {
		let mut list = PortList::default();
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

		assert!(!list.contains_name("test"));
		assert!(!list.contains::<$tp>("test").unwrap());
		assert!(list.contains::<NoType>("inbound0").is_err());
		assert!(list.contains_name("inbound0"));
		assert!(list.contains::<$tp>("inbound0").unwrap());
		assert!(list.contains_name("inoutbound0"));
		assert!(list.contains::<$tp>("inoutbound0").unwrap());
		assert!(list.contains_name("outbound0"));
		assert!(list.contains::<$tp>("outbound0").unwrap());

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

macro_rules! test_connections {
	($tp:ty, $value1: expr, $value2: expr) => {
		let mut list = PortList::default();
		assert!(
			list.insert("outbound", PortVariant::create_outbound($value1))
				.is_ok()
		);
		assert!(
			list.insert("inbound", PortVariant::InBound(BoundInPort::new::<$tp>()))
				.is_ok()
		);

		let mut list2 = PortList::default();
		assert!(
			list2
				.insert("inoutbound", PortVariant::InOutBound(BoundInOutPort::new::<$tp>()))
				.is_ok()
		);
		assert!(
			list2
				.insert("outbound", PortVariant::create_outbound($value1))
				.is_ok()
		);
		assert!(
			list2
				.insert("inbound", PortVariant::InBound(BoundInPort::new::<$tp>()))
				.is_ok()
		);

		let mut invalid = PortList::default();
		assert!(
			invalid
				.insert("invalid", PortVariant::create_inoutbound(NoType))
				.is_ok()
		);

		assert!(
			list.connect_to("notthere", &invalid, "invalid")
				.is_err()
		);
		assert!(
			list.connect_to("inbound", &invalid, "notthere")
				.is_err()
		);
		assert!(
			list.connect_to("inbound", &invalid, "invalid")
				.is_err()
		);
		assert!(
			list2
				.connect_to("inoutbound", &invalid, "invalid")
				.is_err()
		);
		assert!(
			list.connect_to("outbound", &invalid, "invalid")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("notthere", &list, "inbound")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &list2, "notthere")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &list, "inbound")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &list2, "inoutbound")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &list, "outbound")
				.is_err()
		);

		assert!(
			list2
				.connect_to("inoutbound", &list, "outbound")
				.is_ok()
		);
		assert!(
			list.connect_to("inbound", &list2, "inoutbound")
				.is_ok()
		);

		assert_eq!(list.get("inbound").unwrap(), Some($value1));
		assert_eq!(list.sequence_number("inbound"), Ok(1));

		assert!(list.set("outbound", $value2).is_ok());
		assert_eq!(list.get("inbound").unwrap(), Some($value2));
		assert_eq!(list.sequence_number("outbound"), Ok(2));

		// @TODO: is that really ok?
		assert!(
			list.connect_to("inbound", &list2, "inbound")
				.is_ok()
		);
		// @TODO: is that really ok?
		assert!(
			list.connect_to("outbound", &list2, "outbound")
				.is_ok()
		);
	};
}

#[test]
fn list_connection() {
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
		let mut list = PortList::default();
		let mut list2 = create_port_list!(create_inbound_entry!("test", $tp, $value));
		list.append(&mut list2);
	};
}

#[test]
fn list_deref() {
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
		let mut list = create_port_list!(
			create_inbound_entry!("in", $tp, $value),
			create_inoutbound_entry!("inout", $tp, $value),
			create_outbound_entry!("out", $tp, $value),
			create_inoutbound_entry!("empty", $tp),
			create_inbound_entry!("delete1", $tp),
			create_inoutbound_entry!("delete2", $tp, $value),
			create_outbound_entry!("delete3", $tp, $value),
		);

		let entry: (Arc<str>, PortVariant) = create_inbound_entry!("delete1", $tp2, $value2);
		assert_eq!(list.insert(entry.0, entry.1), Err(Error::AlreadyInCollection));
		assert_eq!(list.get::<$tp>("delete1"), Ok(None));

		assert_eq!(list.remove::<$tp>("not_there"), Err(Error::NotFound));

		assert_eq!(list.get::<$tp2>("not_there"), Err(Error::NotFound));
		assert_eq!(list.get::<$tp2>("in"), Err(Error::DataType));
		assert!(list.read::<$tp2>("in").is_err());
		assert!(list.try_read::<$tp2>("in").is_err());
		assert_eq!(list.get::<$tp2>("inout"), Err(Error::DataType));
		assert!(list.read::<$tp2>("inout").is_err());
		assert!(list.try_read::<$tp2>("inout").is_err());
		assert_eq!(list.replace::<$tp2>("inout", $value2), Err(Error::DataType));
		assert_eq!(list.take::<$tp2>("inout"), Err(Error::DataType));
		assert_eq!(list.set::<$tp2>("inout", $value2), Err(Error::DataType));
		assert!(list.write::<$tp2>("inout").is_err());
		assert!(list.try_write::<$tp2>("inout").is_err());
		assert_eq!(list.set::<$tp2>("out", $value2), Err(Error::DataType));
		assert!(list.write::<$tp2>("out").is_err());
		assert!(list.try_write::<$tp2>("out").is_err());

		assert_eq!(list.remove::<$tp2>("delete1"), Err(Error::DataType));
		assert_eq!(list.remove::<$tp>("delete1"), Ok(None));
		assert_eq!(list.remove::<$tp2>("delete2"), Err(Error::DataType));
		assert_eq!(list.remove::<$tp>("delete2"), Ok(Some($value)));
		assert_eq!(list.remove::<$tp2>("delete3"), Err(Error::DataType));
		assert_eq!(list.remove::<$tp>("delete3"), Ok(Some($value)));

		let inout_guard = list.write::<$tp>("inout").unwrap();
		assert!(list.try_read::<$tp>("inout").is_err());
		assert!(list.try_write::<$tp>("inout").is_err());
		assert_eq!(*inout_guard, $value);

		assert!(list.write::<$tp>("empty").is_err());
		assert!(list.try_write::<$tp>("empty").is_err());
	};
}

#[test]
fn list_port_collection_mut() {
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
