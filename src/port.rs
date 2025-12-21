// Copyright © 2025 Stephan Kunz
//! Port.

#![allow(unused)]

use core::{any::Any, ops::Deref};

use alloc::{boxed::Box, sync::Arc};

use crate::{InOutPort, InPort, OutPort, PortBase, any_port::AnyPort};

/// Port.
pub struct Port {
	/// Any type of port
	port: Arc<dyn AnyPort>,
}

impl core::fmt::Debug for Port {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Port")
			.field("port", &self.port)
			.finish_non_exhaustive()
	}
}

impl PartialEq for Port {
	/// Ports are partial equal, if both have the same name & type
	fn eq(&self, other: &Self) -> bool {
		if self.name() == other.name() && self.port.type_id() == other.port.type_id() {
			return true;
		}
		false
	}
}

impl PortBase for Port {
	fn name(&self) -> &'static str {
		self.port.name()
	}
}

impl Port {
	pub fn create_inport<T: 'static + Send + Sync>(name: &'static str) -> Self {
		Self {
			port: Arc::new(InPort::<T>::new(name)),
		}
	}

	pub fn create_inoutport<T: 'static + Send + Sync>(name: &'static str) -> Self {
		Self {
			port: Arc::new(InOutPort::<T>::new(name)),
		}
	}

	pub fn create_outport<T: 'static + Send + Sync>(name: &'static str) -> Self {
		Self {
			port: Arc::new(OutPort::<T>::new(name)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&Port>();
		is_normal::<Port>();
	}
}
