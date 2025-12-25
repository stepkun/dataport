// Copyright © 2025 Stephan Kunz
//! Traits for working with ports and port lists.

use alloc::vec::Vec;

use crate::{
	ConstString,
	any_port::AnyPort,
	error::{Error, Result},
	guards::{PortReadGuard, PortWriteGuard},
	in_out_port::InOutPort,
	port::Port,
};

/// PortBase.
pub trait PortBase {
	#[must_use]
	fn name(&self) -> ConstString;
}

/// Port getter's.
pub trait PortGetter<T>: PortBase {
	/// Returns a clone/copy of the T.
	#[must_use]
	fn get(&self) -> Option<T>
	where
		T: Clone;

	/// Returns an immutable guard to the T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn read(&self) -> Result<PortReadGuard<T>>;

	/// Returns the T, deleting the value.
	#[must_use]
	fn take(&self) -> Option<T>;

	/// Returns an immutable guard to the T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_read(&self) -> Result<PortReadGuard<T>>;
}

/// Port setter's.
pub trait PortSetter<T>: PortBase {
	/// Sets a new value to the T and returns the old T.
	#[must_use]
	fn replace(&self, value: impl Into<T>) -> Option<T>;

	/// Sets a new value to the T.
	fn set(&self, value: impl Into<T>);

	/// Returns a mutable guard to the T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn write(&self) -> Result<PortWriteGuard<T>>;

	/// Returns a mutable guard to the T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_write(&self) -> Result<PortWriteGuard<T>>;
}

/// PortList.
pub trait PortList {
	/// Connects the ouput of src_port to dest_list's dest_port.
	/// # Errors
	/// - [`Error::NotFound`], if one of the ports is not in port list.
	/// - [`Error::SrcAlreadySet`], if dest ports source is already set.
	/// - [`Error::WrongType`], if one of the ports is not the needed port type & type of T.
	fn connect_ports<T: 'static + Send + Sync>(
		&self,
		src_port: impl Into<ConstString>,
		dest_list: &impl PortList,
		dest_port: impl Into<ConstString>,
	) -> Result<()> {
		let src_port = src_port.into();
		if let Some(port) = self.find(src_port.clone()) {
			// src_port must be output
			if let Some(out_port) = port.as_out_port::<T>() {
				// dest_port must be input
				let dest_port = dest_port.into();
				if let Some(port) = dest_list.find(dest_port.clone()) {
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

	/// Returns an immutable guard to the T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn read<T: 'static + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortReadGuard<T>> {
		let port = port.into();
		if let Some(port_) = self.find(port.clone()) {
			// port must be input
			if let Some(in_port) = port_.as_in_port::<T>() {
				(*in_port).read()
			} else {
				Err(Error::WrongType { port })
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
	fn try_read<T: 'static + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortReadGuard<T>> {
		let port = port.into();
		if let Some(port_) = self.find(port.clone()) {
			// port must be input
			if let Some(in_port) = port_.as_in_port::<T>() {
				(*in_port).try_read()
			} else {
				Err(Error::WrongType { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns a mutable guard to the T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn write<T: 'static + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortWriteGuard<T>> {
		let port = port.into();
		if let Some(port_) = self.find(port.clone()) {
			// port must be input
			if let Some(out_port) = port_.as_out_port::<T>() {
				(*out_port).write()
			} else {
				Err(Error::WrongType { port })
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
	fn try_write<T: 'static + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortWriteGuard<T>> {
		let port = port.into();
		if let Some(port_) = self.find(port.clone()) {
			// port must be input
			if let Some(out_port) = port_.as_out_port::<T>() {
				(*out_port).try_write()
			} else {
				Err(Error::WrongType { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Lookup a [`Port`].
	#[must_use]
	fn find(&self, name: impl Into<ConstString>) -> Option<&Port> {
		let name = name.into();
		self.portlist()
			.iter()
			.find(|&port| port.name() == name.clone())
			.map(|v| v as _)
	}

	/// Returns a copy of the value of that port.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn get<T: 'static + Clone + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<Option<T>> {
		let port = port.into();
		if let Some(port_) = self.find(port.clone()) {
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
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn take<T: 'static + Clone + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<Option<T>> {
		let port = port.into();
		if let Some(port_) = self.find(port.clone()) {
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
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not an [`InOutPort`] or not the expected type of T.
	fn propagate<T: 'static>(&self, port: impl Into<ConstString>) -> Result<()> {
		let port = port.into();
		if let Some(port_) = self.find(port.clone()) {
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
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn set<T: 'static + Send + Sync>(&self, port: impl Into<ConstString>, value: T) -> Result<()> {
		let port = port.into();
		if let Some(port_) = self.find(port.clone()) {
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
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn replace<T: 'static + Send + Sync>(&self, port: &'static str, value: T) -> Result<Option<T>> {
		if let Some(port_) = self.find(port) {
			// src_port must be output
			if let Some(out_port) = port_.as_out_port::<T>() {
				Ok(out_port.replace(value))
			} else {
				Err(Error::WrongType { port: port.into() })
			}
		} else {
			Err(Error::NotFound { port: port.into() })
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
		let index = list
			.iter()
			.position(|port| port.name() == name.into());
		index.map(|index| list.remove(index))
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		in_port::InPort,
		out_port::OutPort,
		port_list::{DynamicPortList, StaticPortList},
	};

	use super::*;

	fn return_impl_getter() -> impl PortGetter<i32> {
		InPort::new("port")
	}

	fn use_impl_getter(src: impl PortGetter<i32>) {
		assert!(src.read().is_err());
		assert!(src.get().is_none());
		assert!(src.take().is_none());
	}

	fn return_impl_setter() -> impl PortSetter<i32> {
		OutPort::new("port")
	}

	fn use_impl_setter(src: impl PortSetter<i32>) {
		src.set(22);
		*src.write().unwrap() = 24;
		assert_eq!(src.replace(42).unwrap(), 24);
	}

	fn return_impl_list() -> impl PortList {
		StaticPortList::new([
			Port::create_in_port::<i32>("in"),
			Port::create_out_port::<i32>("out"),
			Port::create_inout_port::<i32>("inout"),
		])
	}

	fn use_impl_list(list: impl PortList) {
		assert!(list.find("port").is_none());
		assert!(list.find("in").is_some());
		assert!(list.find("inout").is_some());
		assert!(list.find("out").is_some());
	}

	fn return_impl_hub() -> impl PortHub {
		let mut list = DynamicPortList::new(Vec::new());
		list.add(Port::create_in_port::<i32>("in"));
		list.add(Port::create_out_port::<i32>("out"));
		list.add(Port::create_inout_port::<i32>("inout"));
		list
	}

	fn use_impl_hub(mut hub: impl PortHub) {
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
	fn dyn_compatibility() {
		let in_port = return_impl_getter();
		use_impl_getter(in_port);

		let out_port = return_impl_setter();
		use_impl_setter(out_port);

		let list = return_impl_list();
		use_impl_list(list);

		let list = return_impl_hub();
		use_impl_list(list);

		let hub = return_impl_hub();
		use_impl_hub(hub);
	}
}
