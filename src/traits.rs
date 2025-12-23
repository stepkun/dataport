// Copyright © 2025 Stephan Kunz
//! Traits for working with ports and port lists.

use alloc::vec::Vec;

use crate::{Error, InOutPort, Port, PortReadGuard, PortWriteGuard, Result, any_port::AnyPort};

/// PortBase.
pub trait PortBase {
	#[must_use]
	fn name(&self) -> &'static str;
}

/// PortDefault.
pub trait PortDefault<T>: PortBase {
	/// Returns a default value for T.
	/// The default implementation returns [`Error::NoDefaultDefined`]
	/// # Errors
	/// - [`Error::NoDefaultDefined`], if no default value is defined
	fn default_value(&self) -> Result<T> {
		Err(Error::NoValueSet { port: self.name() })
	}
}

/// Port getter's.
pub trait PortGetter<T>: PortBase {
	/// Returns an immutable reference to the T.
	fn as_ref(&self) -> Result<PortReadGuard<T>>;

	/// Returns a clone/copy of the T.
	#[must_use]
	fn get(&self) -> Option<T>
	where
		T: Clone;

	/// Returns the T, deleting the value.
	#[must_use]
	fn take(&self) -> Option<T>;
}

/// Port setter's.
pub trait PortSetter<T>: PortBase {
	/// Returns a mutable reference to the T.
	fn as_mut(&self) -> Result<PortWriteGuard<T>>;

	/// Sets a new value to the T and returns the old T.
	#[must_use]
	fn replace(&self, value: impl Into<T>) -> Option<T>;

	/// Sets a new value to the T.
	fn set(&self, value: impl Into<T>);
}

/// PortList.
pub trait PortList {
	/// Connects the ouput of src_port to dest_list's dest_port.
	fn connect_ports<T: 'static + Send + Sync>(
		&self,
		src_port: &'static str,
		dest_list: &impl PortList,
		dest_port: &'static str,
	) -> Result<()> {
		if let Some(port) = self.find(src_port) {
			// src_port must be output
			if let Some(out_port) = port.as_out_port::<T>() {
				// dest_port must be input
				if let Some(port) = dest_list.find(dest_port) {
					if let Some(in_port) = port.as_in_port::<T>() {
						if in_port.src().is_none() {
							let _ = in_port.set_src(out_port.clone());
							Ok(())
						} else {
							Err(Error::SrcAlreadySet { port: dest_port })
						}
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

	/// Returns an immutable reference to the T.
	fn as_ref<T: 'static + Send + Sync>(&self, port: &'static str) -> Result<PortReadGuard<T>> {
		if let Some(port_) = self.find(port) {
			// port must be input
			if let Some(in_port) = port_.as_in_port::<T>() {
				(*in_port).as_ref()
			} else {
				Err(Error::WrongType { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns a mutable reference to the T.
	fn as_mut<T: 'static + Send + Sync>(&self, port: &'static str) -> Result<PortWriteGuard<T>> {
		if let Some(port_) = self.find(port) {
			// port must be input
			if let Some(out_port) = port_.as_out_port::<T>() {
				(*out_port).as_mut()
			} else {
				Err(Error::WrongType { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Lookup a [`Port`].
	#[must_use]
	fn find(&self, name: &str) -> Option<&Port> {
		self.portlist()
			.iter()
			.find(|&port| port.name() == name)
			.map(|v| v as _)
	}

	/// Returns a copy of the value of that port.
	fn get<T: 'static + Clone + Send + Sync>(&self, port: &'static str) -> Result<Option<T>> {
		if let Some(port_) = self.find(port) {
			// port must be input
			if let Some(in_port) = port_.as_in_port::<T>() {
				Ok(in_port.get())
			} else {
				Err(Error::WrongType { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns the value of that port.
	fn take<T: 'static + Clone + Send + Sync>(&self, port: &'static str) -> Result<Option<T>> {
		if let Some(port_) = self.find(port) {
			// port must be input
			if let Some(in_port) = port_.as_in_port::<T>() {
				Ok(in_port.get())
			} else {
				Err(Error::WrongType { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns a reference to the port list.
	#[must_use]
	fn portlist(&self) -> &[Port];

	/// Propagate an inout port's value from in to out.
	fn propagate<T: 'static>(&self, port: &'static str) -> Result<()> {
		if let Some(port_) = self.find(port) {
			let p = &*port_.port();
			let any_port = AnyPort::as_any(p);
			if let Some(inout_port) = any_port.downcast_ref::<InOutPort<T>>() {
				// Now we now this is an InOutPort<T>, return the output part.
				inout_port.propagate();
				Ok(())
			} else {
				Err(Error::WrongType { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Sets the port to the value.
	fn set<T: 'static + Send + Sync>(&self, port: &'static str, value: T) -> Result<()> {
		if let Some(port_) = self.find(port) {
			// src_port must be output
			if let Some(out_port) = port_.as_out_port::<T>() {
				out_port.set(value);
				Ok(())
			} else {
				Err(Error::WrongType { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Replaces the port's value with the `value` and returns the old value.
	fn replace<T: 'static + Send + Sync>(&self, port: &'static str, value: T) -> Result<Option<T>> {
		if let Some(port_) = self.find(port) {
			// src_port must be output
			if let Some(out_port) = port_.as_out_port::<T>() {
				Ok(out_port.replace(value))
			} else {
				Err(Error::WrongType { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
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
		let index = list.iter().position(|port| port.name() == name);
		index.map(|index| list.remove(index))
	}
}
