// Copyright © 2025 Stephan Kunz
//! Implementation of a port providing [`InPort`].

use core::{any::Any, ops::Deref};

use alloc::sync::Arc;

use crate::{
	ConstString, RwLock,
	any_port::AnyPort,
	error::{Error, Result},
	guards::PortReadGuard,
	out_port::OutputPort,
	traits::{InPort, PortBase},
};

/// InPort
pub struct InputPort<T> {
	/// An identifying name of the port, which must be unique for a given item.
	name: ConstString,
	/// The source [`OutPort`] to fetch new values from.
	src: RwLock<Option<Arc<OutputPort<T>>>>,
}

impl<T: 'static + Send + Sync> AnyPort for InputPort<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

impl<T> core::fmt::Debug for InputPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("InputPort")
			.field("name", &self.name)
			//.field("src", &self.src)
			.finish_non_exhaustive()
	}
}

impl<T: 'static> PartialEq for InputPort<T> {
	/// Partial equality of an in port is, if both have the same name & value type
	fn eq(&self, other: &Self) -> bool {
		if self.name == other.name {
			let v1 = self.src.read();
			let v2 = other.src.read();
			if let Some(value1) = &*v1
				&& let Some(value2) = &*v2
			{
				// check type of value1 against type of value2
				if value1.deref().type_id() == value2.deref().type_id() {
					return true;
				}
			} else if v1.is_none() && v2.is_none() {
				return true;
			}
		}
		false
	}
}

impl<T> PortBase for InputPort<T> {
	fn name(&self) -> ConstString {
		self.name.clone()
	}
}

impl<T> InPort<T> for InputPort<T> {
	fn get(&self) -> Option<T>
	where
		T: Clone,
	{
		if let Some(src) = &*self.src.read() {
			src.by_copy()
		} else {
			None
		}
	}

	fn read(&self) -> Result<PortReadGuard<T>> {
		if let Some(src) = &*self.src.read() {
			src.by_ref()
		} else {
			Err(Error::NoSrcSet { port: self.name.clone() })
		}
	}

	fn sequence_id(&self) -> Option<u32> {
		if let Some(src) = &*self.src.read() {
			src.sequence_id()
		} else {
			None
		}
	}

	fn try_read(&self) -> Result<PortReadGuard<T>> {
		if let Some(src) = &*self.src.read() {
			src.try_by_ref()
		} else {
			Err(Error::NoSrcSet { port: self.name.clone() })
		}
	}

	fn take(&self) -> Option<T> {
		if let Some(src) = &*self.src.read() {
			src.by_value()
		} else {
			None
		}
	}

	fn src(&self) -> Option<Arc<OutputPort<T>>> {
		self.src.read().clone()
	}

	fn replace_src(&self, src: impl Into<Arc<OutputPort<T>>>) -> Option<Arc<OutputPort<T>>> {
		self.src.write().replace(src.into())
	}
}

impl<T> InputPort<T> {
	#[must_use]
	pub fn new(name: impl Into<ConstString>) -> Self {
		Self {
			name: name.into(),
			src: RwLock::new(None),
		}
	}

	#[must_use]
	pub fn with_src(name: impl Into<ConstString>, src: impl Into<Arc<OutputPort<T>>>) -> Self {
		Self {
			name: name.into(),
			src: RwLock::new(Some(src.into())),
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::string::String;

	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&InputPort<i32>>();
		is_normal::<InputPort<String>>();
	}
}
