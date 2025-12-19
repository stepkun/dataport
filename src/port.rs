// Copyright © 2025 Stephan Kunz
//! Port.

#![allow(unused)]

use core::{any::Any, ops::Deref};

use alloc::{boxed::Box, sync::Arc};

use crate::{Error, RwLock, any_extensions::AnySendSync};

/// BasePort.
pub trait PortBase {
	fn name(&self) -> &'static str;
}

/// PortDefault.
pub trait PortDefault: PortBase {
	/// Returns a default value for T.
	/// The default implementation returns [`Error::NoDefaultDefined`]
	/// # Errors
	/// - [`Error::NoDefaultDefined`], if no default value is defined
	fn default_value<T>(&self) -> Result<T, Error> {
		Err(Error::NoDefaultDefined { port: self.name() })
	}
}

/// PortGetter's.
pub trait PortGetter: PortDefault {
	/// Returns a reference to the T.
	fn as_ref<T>(&self) -> &T;

	/// Returns a clone/copy of the T.
	fn get<T>(&self) -> T
	where
		T: Clone;
}

/// PortSetter's.
pub trait PortSetter: PortDefault {
	/// Returns a mutable reference to the T.
	fn as_mut<T>(&mut self) -> &mut T;

	/// Sets a new value to the T and returns the old T.
	fn set<T>(&mut self, value: T) -> T;
}

/// Port.
pub struct Port {
	/// An identifying name of the port, which must be unique for a given item.
	name: &'static str,
	value: Option<Box<dyn AnySendSync>>,
}

impl core::fmt::Debug for Port {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Port")
			.field("name", &self.name)
			//.field("value", &self.value)
			.finish_non_exhaustive()
	}
}

impl PartialEq for Port {
	/// Equality of a port is, if both have the same name & type
	fn eq(&self, other: &Self) -> bool {
		if self.name == other.name {
			if let Some(value1) = &self.value
				&& let Some(value2) = &other.value
			{
				// check type of value1 against type of value2
				if value1.deref().type_id() == value2.deref().type_id() {
					return true;
				}
			} else if self.value.is_none() && other.value.is_none() {
				return true;
			}
		}
		false
	}
}

impl PortBase for Port {
	fn name(&self) -> &'static str {
		self.name
	}
}

impl Port {
	pub fn new(name: &'static str) -> Self {
		Self { name, value: None }
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
}
