// Copyright © 2025 Stephan Kunz

#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

extern crate alloc;

use alloc::sync::Arc;

// internal re-export for easy changeability
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};

// re-export macros
pub use dataport_macros::{
	create_inbound_entry, create_inoutbound_entry, create_outbound_entry, create_port_array, create_port_list,
	create_port_map,
};

// flatten
pub use any_port_value::AnyPortValue;
pub use bind::{
	BindCommons, BindIn, BindInOut, BindOut,
	in_out_port::BoundInOutPort,
	in_port::BoundInPort,
	out_port::BoundOutPort,
	port_value::{PortValueReadGuard, PortValueWriteGuard},
};
pub use collections::{
	PortCollection, PortCollectionAccessors, PortCollectionAccessorsCommon, PortCollectionAccessorsMut, PortCollectionMut,
	port_array::EMPTY_PORT_ARRAY, port_array::PortArray, port_list::PortList, port_map::PortMap,
};
//pub use flow::{in_out_port::FlowingInOutPort, in_port::FlowingInPort, out_port::FlowingOutPort};
pub use error::Error;
pub use port_variant::PortVariant;

// internal module structure
mod any_port_value;
mod bind;
mod collections;
mod error;
//mod flow;
mod port_variant;

/// An immutable thread safe `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
type ConstString = Arc<str>;
