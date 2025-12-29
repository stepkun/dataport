// Copyright © 2025 Stephan Kunz
//! Traits for working with ports and lists of ports.

#![allow(unused)]

use core::any::Any;

use alloc::{sync::Arc, vec::Vec};

use crate::{
	ConstString, InputPort, OutputPort, RwLock,
	error::{Error, Result},
	in_out_port::InputOutputPort,
	port::Port,
	port_value::{PortValue, PortValueReadGuard, PortValueWriteGuard},
};

/// The `AnySendSync` trait allows to send data between threads.
pub(crate) trait AnySendSync: Any + Send + Sync {
	/// Convert to Any
	#[must_use]
	fn as_any(&self) -> &dyn Any;

	/// Convert to mut Any
	#[must_use]
	fn as_mut_any(&mut self) -> &mut dyn Any;
}

/// Implementation for any type that has a `static` lifetime and implements [`Send`] and [`Sync`].
impl<T: 'static + Send + Sync> AnySendSync for T {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

pub(crate) trait AnyPort: AnySendSync + core::fmt::Debug + PortBase {
	/// Convert to Any
	#[must_use]
	fn as_any(&self) -> &dyn Any;

	/// Convert to mut Any
	#[must_use]
	fn as_mut_any(&mut self) -> &mut dyn Any;
}

/// Common features for all types of ports.
pub trait PortBase {
	/// Returns an identifying name of the port.
	/// Must be unique within an item providing multiple ports.
	#[must_use]
	fn name(&self) -> ConstString;

	/// Returns the change sequence number, which wraps around to `1` after reaching u32::MAX.
	/// A sequence id of `0` means that the value has never been set or changed.
	fn sequence_number(&self) -> u32;
}

/// Trait for input port types.
pub trait InPort<T>: PortBase {
	/// Returns a clone/copy of the T.
	#[must_use]
	fn get(&self) -> Option<T>
	where
		T: Clone;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn read(&self) -> Result<PortValueReadGuard<T>>;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_read(&self) -> Result<PortValueReadGuard<T>>;
}

/// Trait for output port types.
pub trait OutPort<T>: PortBase {
	/// Sets a new value to the T and returns the old T.
	#[must_use]
	fn replace(&self, value: impl Into<T>) -> Option<T>;

	/// Sets a new value to the T.
	fn set(&self, value: impl Into<T>);

	/// Returns the T, removing it from the port.
	#[must_use]
	fn take(&self) -> Option<T>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn write(&self) -> Result<PortValueWriteGuard<T>>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_write(&self) -> Result<PortValueWriteGuard<T>>;
}

/// Something that provides ports.
pub trait PortProvider {
	/// Lookup a [`Port`].
	#[must_use]
	fn find(&self, name: impl Into<ConstString>) -> Option<&Port>;
}

/// Accessors to ports.
pub trait PortAccessors: PortProvider {
	/// Binds the in port to the out port.
	/// Port `out` is where the value is created, `in` where it is consumed.
	/// # Errors
	/// - [`Error::NotFound`], if one of the ports is not in port list.
	/// - [`Error::PortAlreadyBound`], if in port is already bound.
	/// - [`Error::WrongType`], if one of the ports is not the needed port type & type of T.
	fn bind_to<T: 'static + Send + Sync>(
		&self,
		in_port: impl Into<ConstString>,
		out_list: &impl PortList,
		out_port: impl Into<ConstString>,
	) -> Result<()> {
		// src is where the value is created, dest where it is consumed
		let src_port = out_port.into();
		if let Some(out_port) = out_list.find(src_port.clone()) {
			// src must provide an output value of the wanted type
			if let Some(out_value) = out_port.as_out_value::<T>() {
				let dest_port = in_port.into();
				if let Some(in_port) = self.find(dest_port.clone()) {
					// dest must want input value of the wanted type
					if let Some(input_port) = in_port.port().downcast_ref::<InputPort<T>>() {
						input_port.set_value(out_value);
						Ok(())
					} else if let Some(input_output_port) = in_port
						.port()
						.downcast_ref::<InputOutputPort<T>>()
					{
						todo!(); //input_output_port.set_value(out_value);
						Ok(())
					} else {
						Err(Error::WrongType { port: dest_port })
					}
				} else {
					Err(Error::NotFound { port: dest_port })
				}
			} else {
				Err(Error::WrongType { port: src_port })
			}
		} else {
			Err(Error::NotFound { port: src_port })
		}
	}

	/// Returns a copy of the value of that port.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn get<T: 'static + Clone + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<Option<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value) = port_ref.as_value::<T>() {
				Ok(value.read().get())
			} else {
				Err(Error::ValueNotSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns an immutable guard to the T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn read<T: 'static + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortValueReadGuard<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				PortValueReadGuard::new(port_ref.name(), value_ref.clone())
			} else {
				Err(Error::ValueNotSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns an immutable guard to the T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_read<T: 'static + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortValueReadGuard<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				PortValueReadGuard::try_new(port_ref.name(), value_ref.clone())
			} else {
				Err(Error::ValueNotSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Replaces the port's value with the `value` and returns the old value.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn replace<T: 'static + Send + Sync>(&self, port: &str, value: impl Into<T>) -> Result<Option<T>> {
		if let Some(port_ref) = self.find(port) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				Ok(value_ref.write().replace(value))
			} else {
				Err(Error::ValueNotSet { port: port.into() })
			}
		} else {
			Err(Error::NotFound { port: port.into() })
		}
	}

	/// Sets the port to the value.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn set<T: 'static + Send + Sync>(&self, port: impl Into<ConstString>, value: impl Into<T>) -> Result<()> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				value_ref.write().set(value);
				Ok(())
			} else {
				Err(Error::ValueNotSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns the value of that port.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn take<T: 'static + Clone + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<Option<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				Ok(value_ref.write().take())
			} else {
				Err(Error::ValueNotSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns a mutable guard to the T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn write<T: 'static + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortValueWriteGuard<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				PortValueWriteGuard::new(port_ref.name(), value_ref.clone())
			} else {
				Err(Error::ValueNotSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns a mutable guard to the T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_write<T: 'static + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortValueWriteGuard<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				PortValueWriteGuard::try_new(port_ref.name(), value_ref.clone())
			} else {
				Err(Error::ValueNotSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}
}

/// PortList.
pub trait PortList: PortAccessors {
	/// Returns a reference to the port list.
	#[must_use]
	fn portlist(&self) -> &[Port];
}

/// Implement PortAccessors for anything that implements PortList
impl<T: PortList> PortAccessors for T {}

/// Implement PortProvider for anything that implements PortList
impl<T: PortList> PortProvider for T {
	fn find(&self, name: impl Into<ConstString>) -> Option<&Port> {
		let name = name.into();
		self.portlist()
			.iter()
			.find(|&port| port.name() == name.clone())
			.map(|v| v as _)
	}
}

/// PortHub.
pub trait PortHub: PortList {
	/// Returns a mutable reference to the port list.
	fn portlist_mut(&mut self) -> &mut Vec<Port>;

	/// Adds a port to the portlist.
	fn add(&mut self, port: Port) {
		self.portlist_mut().push(port)
	}

	/// Removes a port from the port list.
	fn remove(&mut self, name: &str) -> Option<Port> {
		let list = self.portlist_mut();
		let index = list
			.iter()
			.position(|port| port.name() == name.into());
		index.map(|index| list.remove(index))
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		in_port::InputPort,
		out_port::OutputPort,
		port_list::{DynamicPortList, StaticPortList},
	};

	use super::*;

	fn return_impl_in_port() -> impl InPort<i32> {
		InputPort::new("port")
	}

	fn use_impl_in_port(src: impl InPort<i32>) {
		assert!(src.read().is_err());
		assert!(src.get().is_none());
	}

	fn return_impl_out_port() -> impl OutPort<i32> {
		OutputPort::new("port")
	}

	fn use_impl_out_port(src: impl OutPort<i32>) {
		src.set(22);
		*src.write().unwrap() = 24;
		assert_eq!(src.replace(42).unwrap(), 24);
		assert_eq!(src.take().unwrap(), 42);
	}

	fn return_impl_port_list() -> impl PortList {
		StaticPortList::new([
			Port::create_in_port::<i32>("in"),
			Port::create_out_port::<i32>("out"),
			Port::create_inout_port::<i32>("inout"),
		])
	}

	fn use_impl_port_list(list: impl PortList) {
		assert!(list.find("port").is_none());
		assert!(list.find("in").is_some());
		assert!(list.find("inout").is_some());
		assert!(list.find("out").is_some());
	}

	fn return_impl_port_hub() -> impl PortHub {
		let mut list = DynamicPortList::new(Vec::new());
		list.add(Port::create_in_port::<i32>("in"));
		list.add(Port::create_out_port::<i32>("out"));
		list.add(Port::create_inout_port::<i32>("inout"));
		list
	}

	fn use_impl_port_hub(mut hub: impl PortHub) {
		assert!(hub.find("port").is_none());
		assert!(hub.find("in").is_some());
		assert!(hub.find("inout").is_some());
		assert!(hub.find("out").is_some());
		hub.remove("in");
		hub.remove("out");
		hub.remove("inout");
		assert!(hub.find("in").is_none());
		assert!(hub.find("inout").is_none());
		assert!(hub.find("out").is_none());
	}

	#[test]
	fn impl_compatibility() {
		let in_port = return_impl_in_port();
		use_impl_in_port(in_port);

		let out_port = return_impl_out_port();
		use_impl_out_port(out_port);

		let list = return_impl_port_list();
		use_impl_port_list(list);

		let list = return_impl_port_hub();
		use_impl_port_list(list);

		let hub = return_impl_port_hub();
		use_impl_port_hub(hub);
	}

	//#[test]
	//fn dyn_compatibility() {
	//	let ip: Arc<dyn InPort<i32>> = InputPort::new("in_port");
	//	let op: Arc<dyn OutPort<i32>> = OutputPort::new("out_port");
	//}
}
