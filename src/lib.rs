// Copyright © 2025 Stephan Kunz
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[doc(hidden)]
#[cfg(feature = "alloc")]
extern crate alloc;

mod any_extensions;
mod error;
mod port;
mod port_list;

// flatten
pub use error::Error;
pub use port::{InputPort, OutputPort, Port};
pub use port_list::{DynamicPortList, PortHub, PortList, StaticPortList};

// A complex type for testing.
#[doc(hidden)]
#[allow(unused)]
#[derive(Clone)]
struct TestStruct {
	a: i32,
	b: f64,
}
