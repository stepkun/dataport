// Copyright © 2026 Stephan Kunz
//! Test Errors public part.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use std::fmt::{Debug, Write};

use dataport::Error;

#[test]
fn error_debug() {
	let err = Error::AlreadyInCollection { name: "test".into() };
	let mut res = String::new();
	write!(res, "{:?}", err);
	assert_eq!(res, String::from("AlreadyInCollection('test')"));

	let err = Error::IsLocked;
	let mut res = String::new();
	write!(res, "{:?}", err);
	assert_eq!(res, String::from("IsLocked"));

	let err = Error::NotFound { name: "test".into() };
	let mut res = String::new();
	write!(res, "{:?}", err);
	assert_eq!(res, String::from("NotFound('test')"));

	let err = Error::NoValueSet;
	let mut res = String::new();
	write!(res, "{:?}", err);
	assert_eq!(res, String::from("NoValueSet"));

	let err = Error::WrongDataType;
	let mut res = String::new();
	write!(res, "{:?}", err);
	assert_eq!(res, String::from("WrongDataType"));

	let err = Error::WrongPortType;
	let mut res = String::new();
	write!(res, "{:?}", err);
	assert_eq!(res, String::from("WrongPortType"));
}

#[test]
fn error_display() {
	let err = Error::AlreadyInCollection { name: "test".into() };
	let mut res = String::new();
	write!(res, "{}", err);
	assert_eq!(res, String::from("a port with the name 'test' is already in the collection"));

	let err = Error::IsLocked;
	let mut res = String::new();
	write!(res, "{}", err);
	assert_eq!(res, String::from("port is currently locked"));

	let err = Error::NotFound { name: "test".into() };
	let mut res = String::new();
	write!(res, "{}", err);
	assert_eq!(res, String::from("port 'test' could not be found in the collection"));

	let err = Error::NoValueSet;
	let mut res = String::new();
	write!(res, "{}", err);
	assert_eq!(res, String::from("no value set for port"));

	let err = Error::WrongDataType;
	let mut res = String::new();
	write!(res, "{}", err);
	assert_eq!(res, String::from("port has a different data type then expected"));

	let err = Error::WrongPortType;
	let mut res = String::new();
	write!(res, "{}", err);
	assert_eq!(res, String::from("port has an incompatible type"));
}
