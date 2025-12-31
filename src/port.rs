// Copyright © 2025 Stephan Kunz
//! A type erased (abstract) port implementation.

use core::any::Any;

use alloc::sync::Arc;

use crate::{
	ConstString,
	in_out_port::InputOutputPort,
	in_port::InputPort,
	out_port::OutputPort,
	port_value::PortValuePtr,
	traits::{AnyPort, PortCommons},
};

/// Port.
#[derive(Clone)]
#[repr(transparent)]
pub struct Port(Arc<dyn AnyPort>);

impl core::fmt::Debug for Port {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("Port")
			.field(&self.0)
			.finish_non_exhaustive()
	}
}

impl<T: 'static + Send + Sync> From<InputOutputPort<T>> for Port {
	fn from(value: InputOutputPort<T>) -> Self {
		Self(Arc::new(value))
	}
}

impl<T: 'static + Send + Sync> From<InputPort<T>> for Port {
	fn from(value: InputPort<T>) -> Self {
		Self(Arc::new(value))
	}
}

impl<T: 'static + Send + Sync> From<OutputPort<T>> for Port {
	fn from(value: OutputPort<T>) -> Self {
		Self(Arc::new(value))
	}
}

impl PartialEq for Port {
	/// Ports are partial equal, if their name, port type & data type are equal.
	fn eq(&self, other: &Self) -> bool {
		if self.name() == other.name()
		  // check the 'dyn AnyPort', not the Arc 
		  && (*self.0).type_id() == (*other.0).type_id()
		{
			return true;
		}
		false
	}
}

impl PortCommons for Port {
	fn name(&self) -> ConstString {
		self.0.name()
	}

	fn sequence_number(&self) -> u32 {
		self.0.sequence_number()
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
	pub fn create_in_port<T: 'static + Send + Sync>(name: impl Into<ConstString>) -> Self {
		Self(Arc::new(InputPort::<T>::new(name)))
	}

	pub fn create_inout_port<T: 'static + Send + Sync>(name: impl Into<ConstString>) -> Self {
		Self(Arc::new(InputOutputPort::<T>::new(name)))
	}

	pub fn create_out_port<T: 'static + Send + Sync>(name: impl Into<ConstString>) -> Self {
		Self(Arc::new(OutputPort::<T>::new(name)))
	}

	pub(crate) fn port(&self) -> &dyn Any {
		&*self.0
	}

	pub(crate) fn as_in_value<T: 'static + Send + Sync>(&self) -> Option<PortValuePtr<T>> {
		let in_port = cast_arc_any_to_in_port::<T>(self.0.clone());
		if let Some(port) = in_port {
			return Some(port.value());
		}

		let in_out_port = cast_arc_any_to_in_out_port::<T>(self.0.clone());
		if let Some(port) = in_out_port {
			return Some(port.value());
		}

		None
	}

	pub(crate) fn as_in_out_port<T: 'static + Send + Sync>(&self) -> Option<Arc<InputOutputPort<T>>> {
		cast_arc_any_to_in_out_port::<T>(self.0.clone())
	}

	pub(crate) fn as_out_value<T: 'static + Send + Sync>(&self) -> Option<PortValuePtr<T>> {
		let out_port = cast_arc_any_to_out_port::<T>(self.0.clone());
		if let Some(port) = out_port {
			return Some(port.value());
		}

		let in_out_port = cast_arc_any_to_in_out_port::<T>(self.0.clone());
		if let Some(port) = in_out_port {
			return Some(port.value());
		}

		None
	}

	pub(crate) fn as_value<T: 'static + Send + Sync>(&self) -> Option<PortValuePtr<T>> {
		if let Some(value) = self.as_in_value() {
			Some(value)
		} else {
			self.as_out_value()
		}
	}

	pub fn get<T: 'static + Clone + Send + Sync>(&self) -> Option<T> {
		if let Some(value) = self.as_in_value::<T>() {
			value.read().get()
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::{string::String, vec::Vec};

	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&Port>();
		is_normal::<Port>();
	}

	const CONST_NAME: &str = "p2";
	static STATIC_NAME: &str = "p3";

	macro_rules! test_casting {
		($tp1:ty, $tp2:ty, $name:expr) => {
			// create with tp1
			let p1 = Port::create_in_port::<$tp1>($name);
			assert!(p1.as_in_value::<$tp1>().is_some());
			assert!(p1.as_in_value::<$tp2>().is_none());
			assert!(p1.as_out_value::<$tp1>().is_none());
			assert!(p1.as_out_value::<$tp2>().is_none());
			assert_eq!(p1.name(), $name.into());

			let p2 = Port::create_out_port::<$tp1>($name);
			assert!(p2.as_in_value::<$tp1>().is_none());
			assert!(p2.as_in_value::<$tp2>().is_none());
			assert!(p2.as_out_value::<$tp1>().is_some());
			assert!(p2.as_out_value::<$tp2>().is_none());
			assert_eq!(p2.name(), $name.into());

			let p3 = Port::create_inout_port::<$tp1>($name);
			assert!(p3.as_in_value::<$tp1>().is_some());
			assert!(p3.as_in_value::<$tp2>().is_none());
			assert!(p3.as_out_value::<$tp1>().is_some());
			assert!(p3.as_out_value::<$tp2>().is_none());
			assert_eq!(p3.name(), $name.into());

			// create with tp2
			let p1 = Port::create_in_port::<$tp2>($name);
			assert!(p1.as_in_value::<$tp2>().is_some());
			assert!(p1.as_in_value::<$tp1>().is_none());
			assert!(p1.as_out_value::<$tp2>().is_none());
			assert!(p1.as_out_value::<$tp1>().is_none());
			assert_eq!(p1.name(), $name.into());

			let p2 = Port::create_out_port::<$tp2>($name);
			assert!(p2.as_in_value::<$tp2>().is_none());
			assert!(p2.as_in_value::<$tp1>().is_none());
			assert!(p2.as_out_value::<$tp2>().is_some());
			assert!(p2.as_out_value::<$tp1>().is_none());
			assert_eq!(p2.name(), $name.into());

			let p3 = Port::create_inout_port::<$tp2>($name);
			assert!(p3.as_in_value::<$tp2>().is_some());
			assert!(p3.as_in_value::<$tp1>().is_none());
			assert!(p3.as_out_value::<$tp2>().is_some());
			assert!(p3.as_out_value::<$tp1>().is_none());
			assert_eq!(p3.name(), $name.into());
		};
	}

	// check casting.
	#[test]
	fn casting() {
		struct MyStruct {
			_f1: i32,
			_f2: f64,
			_f3: String,
			_f4: Vec<f64>,
		}
		let p4_name = String::from("{p4}");
		test_casting!(i32, f64, "p1");
		test_casting!(String, MyStruct, CONST_NAME);
		test_casting!(&str, String, STATIC_NAME);
		test_casting!(Vec<Vec<String>>, Vec<String>, p4_name.as_str());
	}
}
