// Copyright © 2025 Stephan Kunz
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[doc(hidden)]
extern crate alloc;

mod error;
mod in_out_port;
mod in_port;
mod out_port;
mod port;
mod port_data;
mod port_list;
mod port_value;
mod sequence_number;
mod traits;

use alloc::sync::Arc;

// internal re-export for easy changeability
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// An immutable thread safe `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
type ConstString = Arc<str>;

// flatten
pub use error::Error;
pub use in_out_port::InputOutputPort;
pub use in_port::InputPort;
pub use out_port::OutputPort;
pub use port::Port;
pub use port_list::{DynamicPortList, StaticPortList};
pub use port_value::{PortValueReadGuard, PortValueWriteGuard};
pub use traits::{InPort, OutPort, PortAccessors, PortBase, PortHub, PortList, PortProvider};
// re-exports:
//pub use dataport_macros::???;
