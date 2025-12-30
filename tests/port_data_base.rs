// Copyright © 2025 Stephan Kunz
//! Test [`PortDataBase`]

#![allow(unused)]

use std::f64::consts::PI;

use dataport::*;

const CONST_NAME: &str = "p2";
static STATIC_NAME: &str = "p3";

#[test]
#[allow(clippy::cognitive_complexity)]
fn access() {
	let mut pb = PortDataBase::default();
	assert!(!pb.contains_key("test"));
	assert!(pb.port("test").is_err());
	assert!(!pb.contains::<i32>("test").unwrap());
	assert!(pb.get::<i32>("test").is_err());
	assert!(pb.read::<i32>("test").is_err());
	assert!(pb.try_read::<i32>("test").is_err());
	assert!(pb.sequence_number("test").is_err());
	assert!(pb.update::<i32>("test", 42).is_err());
	assert!(pb.write::<i32>("test").is_err());
	assert!(pb.try_write::<i32>("test").is_err());
	assert!(pb.delete::<i32>("test").is_err());

	assert!(pb.create::<i32>("test", 42).is_ok());
	assert!(pb.contains_key("test"));
	assert!(pb.port("test").is_ok());
	assert!(pb.contains::<i32>("test").unwrap());
	assert_eq!(pb.get::<i32>("test").unwrap().unwrap(), 42);
	assert_eq!(*pb.read::<i32>("test").unwrap(), 42);
	assert_eq!(*pb.try_read::<i32>("test").unwrap(), 42);
	assert_eq!(pb.sequence_number("test").unwrap(), 1);

	assert!(pb.update::<i32>("test", 24).is_ok());
	assert_eq!(pb.get::<i32>("test").unwrap().unwrap(), 24);
	assert_eq!(pb.sequence_number("test").unwrap(), 2);

	{
		let guard = pb.read::<i32>("test").unwrap();
		assert!(pb.read::<i32>("test").is_ok());
		assert!(pb.try_read::<i32>("test").is_ok());
		assert!(pb.try_write::<i32>("test").is_err());
	}
	{
		let mut guard = pb.write("test").unwrap();
		*guard = 42;
		assert!(pb.try_read::<i32>("test").is_err());
		assert!(pb.try_write::<i32>("test").is_err());
	}

	assert_eq!(pb.get::<i32>("test").unwrap().unwrap(), 42);
	assert_eq!(pb.sequence_number("test").unwrap(), 3);

	assert!(pb.delete::<i32>("test").is_ok());
}
