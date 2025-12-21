// Copyright © 2025 Stephan Kunz
//! Traits for working with ports.

use alloc::vec::Vec;

use crate::{Error, Port, PortReadGuard, PortWriteGuard, Result};

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
	/// Returns a reference to the T.
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
	/// Returns a reference to the port list.
	fn portlist(&self) -> &[Port];

	/// Lookup a [`Port`].
	#[must_use]
	fn find(&self, name: &str) -> Option<&Port> {
		self.portlist()
			.iter()
			.find(|&port| port.name() == name)
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
		let index = list.iter().position(|port| port.name() == name);
		index.map(|index| list.remove(index))
	}
}
