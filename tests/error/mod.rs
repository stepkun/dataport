// Copyright © 2026 Stephan Kunz
//! Test Errors public part.

#![allow(missing_docs)]

use std::fmt::Write;

use dataport::Error;

#[test]
fn error_debug_display() {
	let err = Error::AlreadyInCollection;
	let mut res = String::new();
	assert!(write!(res, "{}", err).is_ok());
	assert_eq!(res, String::from("a port with that name is already in the collection"));

	let err = Error::IsLocked;
	let mut res = String::new();
	assert!(write!(res, "{}", err).is_ok());
	assert_eq!(res, String::from("port is currently locked"));

	let err = Error::NotFound;
	let mut res = String::new();
	assert!(write!(res, "{}", err).is_ok());
	assert_eq!(res, String::from("port 'self' could not be found in the collection"));

	let err = Error::OtherNotFound;
	let mut res = String::new();
	assert!(write!(res, "{}", err).is_ok());
	assert_eq!(res, String::from("port 'other' could not be found in the collection"));

	let err = Error::NoValueSet;
	let mut res = String::new();
	assert!(write!(res, "{}", err).is_ok());
	assert_eq!(res, String::from("no value set for port"));

	let err = Error::DataType;
	let mut res = String::new();
	assert!(write!(res, "{}", err).is_ok());
	assert_eq!(res, String::from("port has a different data type then expected"));

	let err = Error::PortType;
	let mut res = String::new();
	assert!(write!(res, "{}", err).is_ok());
	assert_eq!(res, String::from("port has an incompatible type"));
}
