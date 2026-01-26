// Copyright © 2026 Stephan Kunz
//! Test [`PortArray`]s public API.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use core::f64::consts::PI;

use std::fmt::{Debug, Write};

use dataport::{
	BoundInOutPort, BoundInPort, BoundOutPort, Error, PortArray, PortCollection, PortCollectionAccessors,
	PortCollectionAccessorsMut, PortVariant, create_inbound_entry, create_inoutbound_entry, create_outbound_entry,
	create_port_array,
};

macro_rules! test_creation {
	($tp:ty, $value: expr) => {
		let mut array = PortArray::from([
			("inbound0".into(), PortVariant::InBound(BoundInPort::new::<$tp>())),
			("outbound0".into(), PortVariant::OutBound(BoundOutPort::new::<$tp>())),
			(
				"inoutbound0".into(),
				PortVariant::InOutBound(BoundInOutPort::new::<$tp>()),
			),
			("inbound1".into(), PortVariant::InBound(BoundInPort::with_value($value))),
			(
				"outbound1".into(),
				PortVariant::OutBound(BoundOutPort::with_value($value)),
			),
			(
				"inoutbound1".into(),
				PortVariant::InOutBound(BoundInOutPort::with_value($value)),
			),
			(
				"outbound1".into(),
				PortVariant::OutBound(BoundOutPort::with_value($value)),
			),
			(
				"inoutbound1".into(),
				PortVariant::InOutBound(BoundInOutPort::with_value($value)),
			),
			("inbound2".into(), PortVariant::create_inbound($value)),
			("outbound2".into(), PortVariant::create_outbound($value)),
			("inoutbound2".into(), PortVariant::create_inoutbound($value)),
		]);

		assert!(array.find("inbound").is_none());
		assert_eq!(
			array.sequence_number("inbound"),
			Err(Error::NotFound { name: "inbound".into() })
		);
		assert!(array.find("outbound").is_none());
		assert_eq!(
			array.sequence_number("outbound"),
			Err(Error::NotFound {
				name: "outbound".into()
			})
		);
		assert!(array.find_mut("inoutbound").is_none());
		assert_eq!(
			array.sequence_number("inoutbound"),
			Err(Error::NotFound {
				name: "inoutbound".into()
			})
		);

		assert!(array.find("inbound0").is_some());
		assert_eq!(array.sequence_number("inbound0"), Ok(0));
		assert!(array.find("inbound1").is_some());
		assert_eq!(array.sequence_number("inbound1"), Ok(1));
		assert!(array.find("inbound2").is_some());
		assert_eq!(array.sequence_number("inbound2"), Ok(1));

		assert!(array.find("outbound0").is_some());
		assert_eq!(array.sequence_number("outbound0"), Ok(0));
		assert!(array.find("outbound1").is_some());
		assert_eq!(array.sequence_number("outbound1"), Ok(1));
		assert!(array.find("outbound2").is_some());
		assert_eq!(array.sequence_number("outbound1"), Ok(1));

		assert!(array.find_mut("inoutbound0").is_some());
		assert_eq!(array.sequence_number("inoutbound0"), Ok(0));
		assert!(array.find_mut("inoutbound1").is_some());
		assert_eq!(array.sequence_number("inoutbound1"), Ok(1));
		assert!(array.find_mut("inoutbound2").is_some());
		assert_eq!(array.sequence_number("inoutbound2"), Ok(1));
	};
}

#[test]
fn array_creation() {
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
		let mut array = PortArray::from([
			("inbound0".into(), PortVariant::InBound(BoundInPort::new::<$tp>())),
			("outbound0".into(), PortVariant::OutBound(BoundOutPort::new::<$tp>())),
			(
				"inoutbound0".into(),
				PortVariant::InOutBound(BoundInOutPort::new::<$tp>()),
			),
			("inbound1".into(), PortVariant::create_inbound($value1)),
			("outbound1".into(), PortVariant::create_outbound($value1)),
			("inoutbound1".into(), PortVariant::create_inoutbound($value1)),
		]);

		assert!(!array.contains_name("test"));
		assert!(!array.contains::<$tp>("test").unwrap());
		assert!(array.contains::<NoType>("inbound0").is_err());
		assert!(array.contains_name("inbound0"));
		assert!(array.contains::<$tp>("inbound0").unwrap());
		assert!(array.contains_name("inoutbound0"));
		assert!(array.contains::<$tp>("inoutbound0").unwrap());
		assert!(array.contains_name("outbound0"));
		assert!(array.contains::<$tp>("outbound0").unwrap());

		assert!(array.get::<$tp>("test").is_err());
		assert_eq!(array.get::<$tp>("inbound0").unwrap(), None);
		assert!(array.get::<$tp>("outbound0").is_err());
		assert_eq!(array.get::<$tp>("inoutbound0").unwrap(), None);
		assert_eq!(array.get::<$tp>("inbound1").unwrap(), Some($value1));
		assert!(array.get::<$tp>("outbound1").is_err());
		assert_eq!(array.get::<$tp>("inoutbound1").unwrap(), Some($value1));

		assert!(array.read::<$tp>("test").is_err());
		assert!(array.read::<$tp>("inbound0").is_err());
		assert!(array.read::<$tp>("inoutbound0").is_err());
		assert!(array.read::<$tp>("outbound0").is_err());
		assert_eq!(*array.read::<$tp>("inbound1").unwrap(), $value1);
		assert!(array.read::<$tp>("outbound1").is_err());
		assert_eq!(*array.read::<$tp>("inoutbound1").unwrap(), $value1);

		assert!(array.try_read::<$tp>("test").is_err());
		assert!(array.try_read::<$tp>("inbound0").is_err());
		assert!(array.try_read::<$tp>("inoutbound0").is_err());
		assert!(array.try_read::<$tp>("outbound0").is_err());
		assert_eq!(*array.try_read::<$tp>("inbound1").unwrap(), $value1);
		assert!(array.try_read::<$tp>("outbound1").is_err());
		assert_eq!(*array.try_read::<$tp>("inoutbound1").unwrap(), $value1);

		assert!(array.set("test", $value2).is_err());
		assert!(array.set("inbound0", $value2).is_err());
		assert!(array.set("outbound0", $value2).is_ok());
		assert!(array.set("inoutbound0", $value2).is_ok());
		assert_eq!(*array.read::<$tp>("inoutbound0").unwrap(), $value2);
		assert!(array.set("inbound1", $value2).is_err());
		assert!(array.set("outbound1", $value2).is_ok());
		assert!(array.set("inoutbound1", $value2).is_ok());
		assert_eq!(array.get::<$tp>("inoutbound1").unwrap(), Some($value2));

		{
			assert!(array.write::<$tp>("test").is_err());
			assert!(array.write::<$tp>("inbound0").is_err());
			let mut g_out = array.write::<$tp>("outbound0").unwrap();
			assert_eq!(*g_out, $value2);
			*g_out = $value1;
			assert_eq!(*g_out, $value1);
			let mut g_inout = array.write::<$tp>("inoutbound0").unwrap();
			assert_eq!(*g_inout, $value2);
			*g_inout = $value1;
			assert_eq!(*g_inout, $value1);
			assert!(array.write::<$tp>("inbound1").is_err());
			let mut g_out = array.write::<$tp>("outbound1").unwrap();
			assert_eq!(*g_out, $value2);
			*g_out = $value1;
			assert_eq!(*g_out, $value1);
			let mut g_inout = array.write::<$tp>("inoutbound1").unwrap();
			assert_eq!(*g_inout, $value2);
			*g_inout = $value1;
			assert_eq!(*g_inout, $value1);
		}
		{
			assert!(array.try_write::<$tp>("test").is_err());
			assert!(array.try_write::<$tp>("inbound0").is_err());
			let mut g_out = array.try_write::<$tp>("outbound0").unwrap();
			assert_eq!(*g_out, $value1);
			*g_out = $value2;
			assert_eq!(*g_out, $value2);
			let mut g_inout = array.try_write::<$tp>("inoutbound0").unwrap();
			assert_eq!(*g_inout, $value1);
			*g_inout = $value2;
			assert_eq!(*g_inout, $value2);
			assert!(array.try_write::<$tp>("inbound1").is_err());
			let mut g_out = array.try_write::<$tp>("outbound1").unwrap();
			assert_eq!(*g_out, $value1);
			*g_out = $value2;
			assert_eq!(*g_out, $value2);
			let mut g_inout = array.try_write::<$tp>("inoutbound1").unwrap();
			assert_eq!(*g_inout, $value1);
			*g_inout = $value2;
			assert_eq!(*g_inout, $value2);
		}
		{
			assert!(array.replace::<$tp>("test", $value1).is_err());
			assert!(array.replace::<$tp>("inbound0", $value1).is_err());
			assert!(
				array
					.replace::<$tp>("outbound0", $value1)
					.is_err()
			);
			assert_eq!(
				array
					.replace::<$tp>("inoutbound0", $value1)
					.unwrap(),
				Some($value2)
			);
			assert!(array.replace::<$tp>("inbound1", $value1).is_err());
			assert!(
				array
					.replace::<$tp>("outbound1", $value1)
					.is_err()
			);
			assert_eq!(
				array
					.replace::<$tp>("inoutbound1", $value1)
					.unwrap(),
				Some($value2)
			);
		}
		{
			assert!(array.take::<$tp>("test").is_err());
			assert!(array.take::<$tp>("inbound0").is_err());
			assert!(array.take::<$tp>("outbound0").is_err());
			assert_eq!(array.take::<$tp>("inoutbound0").unwrap(), Some($value1));
			assert_eq!(array.take::<$tp>("inoutbound0").unwrap(), None);
			assert!(array.take::<$tp>("inbound1").is_err());
			assert!(array.take::<$tp>("outbound1").is_err());
			assert_eq!(array.take::<$tp>("inoutbound1").unwrap(), Some($value1));
			assert_eq!(array.take::<$tp>("inoutbound1").unwrap(), None);
		}
	};
}

#[test]
fn array_accessors() {
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
		let mut array = PortArray::from([
			("outbound".into(), PortVariant::create_outbound($value1)),
			("inbound".into(), PortVariant::InBound(BoundInPort::new::<$tp>())),
		]);
		let mut array2 = PortArray::from([
			(
				"inoutbound".into(),
				PortVariant::InOutBound(BoundInOutPort::new::<$tp>()),
			),
			("outbound".into(), PortVariant::create_outbound($value1)),
			("inbound".into(), PortVariant::InBound(BoundInPort::new::<$tp>())),
		]);
		let mut invalid = PortArray::from([("invalid".into(), PortVariant::create_inoutbound(NoType))]);

		assert!(
			array
				.connect_to("notthere", &invalid, "invalid")
				.is_err()
		);
		assert!(
			array
				.connect_to("inbound", &invalid, "notthere")
				.is_err()
		);
		assert!(
			array
				.connect_to("inbound", &invalid, "invalid")
				.is_err()
		);
		assert!(
			array2
				.connect_to("inoutbound", &invalid, "invalid")
				.is_err()
		);
		assert!(
			array
				.connect_to("outbound", &invalid, "invalid")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("notthere", &array, "inbound")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &array2, "notthere")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &array, "inbound")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &array2, "inoutbound")
				.is_err()
		);
		assert!(
			invalid
				.connect_to("invalid", &array, "outbound")
				.is_err()
		);

		assert!(
			array2
				.connect_to("inoutbound", &array, "outbound")
				.is_ok()
		);
		assert!(
			array
				.connect_to("inbound", &array2, "inoutbound")
				.is_ok()
		);

		assert_eq!(array.get("inbound").unwrap(), Some($value1));

		assert!(array.set("outbound", $value2).is_ok());
		assert_eq!(array.get("inbound").unwrap(), Some($value2));

		// @TODO: is that really ok?
		assert!(
			array
				.connect_to("inbound", &array2, "inbound")
				.is_ok()
		);
		// @TODO: is that really ok?
		assert!(
			array
				.connect_to("outbound", &array2, "outbound")
				.is_ok()
		);
	};
}

#[test]
fn array_connection() {
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

macro_rules! test_deref_debug {
	($tp:ty, $value: expr) => {
		let array = create_port_array!(create_inbound_entry!("test", $tp, $value));
		assert_eq!(array.len(), 1);
		let mut expected = String::new();
		assert!(
			write!(
				expected,
				"PortArray([(\"test\", InBound(BoundInPort(RwLock {{ data: (PortValue(Some({:?})), SequenceNumber(1)) }})))])",
				$value
			)
			.is_ok()
		);

		let mut res = String::new();
		assert!(write!(res, "{:?}", array).is_ok());
		assert_eq!(res, expected,);
	};
}

#[test]
fn array_deref_debug() {
	test_deref_debug!(bool, true);
	test_deref_debug!(i32, 42);
	test_deref_debug!(f64, PI);
	test_deref_debug!(&str, "str");
	test_deref_debug!(String, String::from("string"));
	test_deref_debug!(Vec<i32>, vec![1, 2, 3]);
	test_deref_debug!(Vec<&str>, vec!["1", "2", "3"]);
	test_deref_debug!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		]
	);
	test_deref_debug!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}

macro_rules! test_port_collection {
	($tp:ty, $value: expr, $tp2:ty, $value2: expr) => {
		let mut array = create_port_array!(
			create_inbound_entry!("in", $tp, $value),
			create_inoutbound_entry!("inout", $tp, $value),
			create_outbound_entry!("out", $tp, $value),
			create_inoutbound_entry!("empty", $tp),
		);

		assert_eq!(
			array.get::<$tp2>("not_there"),
			Err(Error::NotFound {
				name: "not_there".into()
			})
		);
		assert_eq!(array.get::<$tp2>("in"), Err(Error::WrongDataType));
		assert!(array.read::<$tp2>("in").is_err());
		assert!(array.try_read::<$tp2>("in").is_err());
		assert_eq!(array.get::<$tp2>("inout"), Err(Error::WrongDataType));
		assert!(array.read::<$tp2>("inout").is_err());
		assert!(array.try_read::<$tp2>("inout").is_err());
		assert_eq!(array.replace::<$tp2>("inout", $value2), Err(Error::WrongDataType));
		assert_eq!(array.take::<$tp2>("inout"), Err(Error::WrongDataType));
		assert_eq!(array.set::<$tp2>("inout", $value2), Err(Error::WrongDataType));
		assert!(array.write::<$tp2>("inout").is_err());
		assert!(array.try_write::<$tp2>("inout").is_err());
		assert_eq!(array.set::<$tp2>("out", $value2), Err(Error::WrongDataType));
		assert!(array.write::<$tp2>("out").is_err());
		assert!(array.try_write::<$tp2>("out").is_err());

		let inout_guard = array.write::<$tp>("inout").unwrap();
		assert!(array.try_read::<$tp>("inout").is_err());
		assert!(array.try_write::<$tp>("inout").is_err());
		assert_eq!(*inout_guard, $value);

		assert!(array.write::<$tp>("empty").is_err());
		assert!(array.try_write::<$tp>("empty").is_err());
	};
}

#[test]
fn array_port_collection_mut() {
	test_port_collection!(bool, true, i32, 42);
	test_port_collection!(i32, 42, f64, PI);
	test_port_collection!(f64, PI, &str, "str");
	test_port_collection!(&str, "str", String, String::from("string"));
	test_port_collection!(String, String::from("string"), Vec<i32>, vec![1, 2, 3]);
	test_port_collection!(Vec<i32>, vec![1, 2, 3], Vec<&str>, vec!["1", "2", "3"]);
	test_port_collection!(Vec<&str>, vec!["1", "2", "3"], bool, true);
	test_port_collection!(
		Vec<String>,
		vec![
			String::from("1"),
			String::from("2"),
			String::from("3")
		],
		bool,
		true
	);
	test_port_collection!(Vec<Vec<f64>>, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]], bool, true);
}
