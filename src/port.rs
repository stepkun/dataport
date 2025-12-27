// Copyright © 2025 Stephan Kunz
//! A type erased (abstract) port implementation.

use core::any::Any;

use alloc::sync::Arc;

use crate::{
	ConstString, any_port::AnyPort, in_out_port::InputOutputPort, in_port::InputPort, out_port::OutputPort, traits::PortBase,
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

impl PortBase for Port {
	fn name(&self) -> ConstString {
		self.port.name()
	}
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

	pub fn as_in_out_port<T: 'static + Send + Sync>(&self) -> Option<Arc<InputOutputPort<T>>> {
		// helper function to downcast the `Arc<dyn Any>` to `Arc<InPort<T>>`
		fn cast_arc_any_to_in_out_port<T: 'static + Send + Sync>(
			any_value: Arc<dyn Any + Send + Sync>,
		) -> Option<Arc<InputOutputPort<T>>> {
			any_value.downcast::<InputOutputPort<T>>().ok()
		}

		let in_out_port = cast_arc_any_to_in_out_port::<T>(self.port.clone());
		if in_out_port.is_some() {
			return in_out_port;
		}
		None
	}

	pub(crate) fn as_in_port<T: 'static + Send + Sync>(&self) -> Option<Arc<InputPort<T>>> {
		// helper function to downcast the `Arc<dyn Any>` to `Arc<InPort<T>>`
		fn cast_arc_any_to_in_port<T: 'static + Send + Sync>(
			any_value: Arc<dyn Any + Send + Sync>,
		) -> Option<Arc<InputPort<T>>> {
			any_value.downcast::<InputPort<T>>().ok()
		}

		let in_port = cast_arc_any_to_in_port::<T>(self.port.clone());
		if in_port.is_some() {
			return in_port;
		}

		let any_port = AnyPort::as_any(&*self.port);
		if let Some(inout_port) = any_port.downcast_ref::<InputOutputPort<T>>() {
			// Now we now this is an InOutPort<T>, return the input part.
			return Some(inout_port.input());
		}
		None
	}

	pub(crate) fn as_out_port<T: 'static + Send + Sync>(&self) -> Option<Arc<OutputPort<T>>> {
		// helper function to downcast the `Arc<dyn Any>` to `Arc<OutPort<T>>`
		fn cast_arc_any_to_out_port<T: 'static + Send + Sync>(
			any_value: Arc<dyn Any + Send + Sync>,
		) -> Option<Arc<OutputPort<T>>> {
			any_value.downcast::<OutputPort<T>>().ok()
		}

		let x = cast_arc_any_to_out_port::<T>(self.port.clone());
		if x.is_some() {
			return x;
		}

		let any_port = AnyPort::as_any(&*self.port);
		if let Some(inout_port) = any_port.downcast_ref::<InputOutputPort<T>>() {
			// Now we now this is an InOutPort<T>, return the output part.
			return Some(inout_port.output());
		}
		None
	}

	pub(crate) fn port(&self) -> Arc<dyn AnyPort> {
		self.port.clone()
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
}
