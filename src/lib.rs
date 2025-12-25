// Copyright © 2025 Stephan Kunz
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[doc(hidden)]
extern crate alloc;

mod any_port;
mod error;
mod guards;
mod in_out_port;
mod in_port;
mod out_port;
mod port;
mod port_list;
mod traits;

use alloc::sync::Arc;

// internal re-export for easy changeability
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// An immutable thread safe `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
type ConstString = Arc<str>;

// flatten
pub use error::Error;
pub use in_out_port::InOutPort;
pub use in_port::InPort;
pub use out_port::OutPort;
pub use port::Port;
pub use port_list::{DynamicPortList, StaticPortList};
pub use traits::{PortGetter, PortHub, PortList, PortSetter};
// re-exports:
//pub use dataport_macros::???;
