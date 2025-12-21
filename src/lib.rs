// Copyright © 2025 Stephan Kunz
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[doc(hidden)]
extern crate alloc;

mod any_extensions;
mod error;
mod guards;
mod in_out_port;
mod in_port;
mod out_port;
mod port;
mod port_list;

// flatten
pub use error::{Error, Result};
pub use guards::{PortReadGuard, PortWriteGuard};
pub use in_out_port::InOutPort;
pub use in_port::InPort;
pub use out_port::OutPort;
pub use port::{Port, PortBase, PortDefault, PortGetter, PortSetter};
pub use port_list::{DynamicPortList, PortHub, PortList, StaticPortList};

// re-exports:
//pub use dataport_macros::???;
// re-export `RwLock` for easy changeability
pub(crate) use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};

// A complex type for testing.
#[doc(hidden)]
#[allow(unused)]
#[derive(Clone)]
struct TestStruct {
	a: i32,
	b: f64,
}
