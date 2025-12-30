// Copyright © 2025 Stephan Kunz
//! A type erased (abstract) port implementation.

#![allow(unused)]

use core::{any::Any, ops::Deref};

use alloc::sync::Arc;

use crate::{
	ConstString, RwLock,
	in_out_port::InputOutputPort,
	in_port::InputPort,
	out_port::OutputPort,
	port_value::PortValue,
	traits::{AnyPort, PortCommons},
};

/// Port.
#[derive(Clone)]
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

impl<T: 'static + Send + Sync> From<InputOutputPort<T>> for Port {
	fn from(value: InputOutputPort<T>) -> Self {
		Self { port: Arc::new(value) }
	}
}

impl PartialEq for Port {
	/// Ports are partial equal, if their name, port type & data type are equal.
	fn eq(&self, other: &Self) -> bool {
		if self.name() == other.name()
		  // check the 'dyn AnyPort', not the Arc 
		  && (*self.port).type_id() == (*other.port).type_id()
		{
			return true;
		}
		false
	}
}

impl PortCommons for Port {
	fn name(&self) -> ConstString {
		self.port.name()
	}

	fn sequence_number(&self) -> u32 {
		self.port.sequence_number()
	}
}

// helper function to downcast the `Arc<dyn Any>` to `Arc<InPort<T>>`
fn cast_arc_any_to_in_port<T: 'static + Send + Sync>(any_value: Arc<dyn Any + Send + Sync>) -> Option<Arc<InputPort<T>>> {
	any_value.downcast::<InputPort<T>>().ok()
}

// helper function to downcast the `Arc<dyn Any>` to `Arc<InOutPort<T>>`
fn cast_arc_any_to_in_out_port<T: 'static + Send + Sync>(
	any_value: Arc<dyn Any + Send + Sync>,
) -> Option<Arc<InputOutputPort<T>>> {
	any_value.downcast::<InputOutputPort<T>>().ok()
}

// helper function to downcast the `Arc<dyn Any>` to `Arc<OutPort<T>>`
fn cast_arc_any_to_out_port<T: 'static + Send + Sync>(any_value: Arc<dyn Any + Send + Sync>) -> Option<Arc<OutputPort<T>>> {
	any_value.downcast::<OutputPort<T>>().ok()
}

impl Port {
	pub fn create_in_port<T: 'static + Send + Sync>(name: &'static str) -> Self {
		Self {
			port: Arc::new(InputPort::<T>::new(name)),
		}
	}

	pub fn create_inout_port<T: 'static + Send + Sync>(name: &'static str) -> Self {
		Self {
			port: Arc::new(InputOutputPort::<T>::new(name)),
		}
	}

	pub fn create_out_port<T: 'static + Send + Sync>(name: &'static str) -> Self {
		Self {
			port: Arc::new(OutputPort::<T>::new(name)),
		}
	}

	pub(crate) fn port(&self) -> &dyn Any {
		&*self.port
	}

	pub(crate) fn as_in_value<T: 'static + Send + Sync>(&self) -> Option<Arc<RwLock<PortValue<T>>>> {
		let in_port = cast_arc_any_to_in_port::<T>(self.port.clone());
		if let Some(port) = in_port {
			return Some(port.value());
		}

		let in_out_port = cast_arc_any_to_in_out_port::<T>(self.port.clone());
		if let Some(port) = in_out_port {
			return Some(port.value());
		}

		None
	}

	pub fn as_in_out_port<T: 'static + Send + Sync>(&self) -> Option<Arc<InputOutputPort<T>>> {
		cast_arc_any_to_in_out_port::<T>(self.port.clone())
	}

	pub(crate) fn as_out_value<T: 'static + Send + Sync>(&self) -> Option<Arc<RwLock<PortValue<T>>>> {
		let out_port = cast_arc_any_to_out_port::<T>(self.port.clone());
		if let Some(port) = out_port {
			return Some(port.value());
		}

		let in_out_port = cast_arc_any_to_in_out_port::<T>(self.port.clone());
		if let Some(port) = in_out_port {
			return Some(port.value());
		}

		None
	}

	pub(crate) fn as_value<T: 'static + Send + Sync>(&self) -> Option<Arc<RwLock<PortValue<T>>>> {
		if let Some(value) = self.as_in_value() {
			Some(value)
		} else {
			self.as_out_value()
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

	/*
	   // check casting.
	   #[test]
	   fn casting() {
		   let p1 = Port::create_in_port::<i32>("in");
		   assert!(p1.as_out_port::<f64>().is_none());
		   assert!(p1.as_out_port::<i32>().is_none());
		   assert!(p1.as_in_port::<f64>().is_none());
		   assert!(p1.as_in_port::<i32>().is_some());
		   let in_port: Arc<InputPort<i32>> = p1.as_in_port().unwrap();
		   assert_eq!(in_port.name(), "in".into());

		   let p2 = Port::create_out_port::<i32>("out");
		   assert!(p2.as_out_port::<f64>().is_none());
		   assert!(p2.as_out_port::<i32>().is_some());
		   let out_port: Arc<OutputPort<i32>> = p2.as_out_port().unwrap();
		   assert_eq!(out_port.name(), "out".into());

		   let p3 = Port::create_inout_port::<i32>("inout");
		   assert!(p3.as_in_port::<f64>().is_none());
		   assert!(p3.as_out_port::<f64>().is_none());
		   let in_port: Arc<InputPort<i32>> = p3.as_in_port().unwrap();
		   assert_eq!(in_port.name(), "inout".into());
		   let out_port: Arc<OutputPort<i32>> = p3.as_out_port().unwrap();
		   assert_eq!(out_port.name(), "inout".into());
	   }
	*/
}
