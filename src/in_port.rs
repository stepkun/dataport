// Copyright © 2025 Stephan Kunz
//! Port.

use core::{any::Any, ops::Deref};

use alloc::sync::Arc;

use crate::{
	ConstString, RwLock,
	any_port::AnyPort,
	error::{Error, Result},
	guards::PortReadGuard,
	out_port::OutPort,
	traits::{PortBase, PortGetter},
};

/// InPort
pub struct InPort<T> {
	/// An identifying name of the port, which must be unique for a given item.
	name: ConstString,
	/// The source [`OutPort`] to fetch new values from.
	src: RwLock<Option<Arc<OutPort<T>>>>,
}

impl<T: 'static + Send + Sync> AnyPort for InPort<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

impl<T> core::fmt::Debug for InPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("InPort")
			.field("name", &self.name)
			//.field("src", &self.src)
			.finish_non_exhaustive()
	}
}

impl<T: 'static> PartialEq for InPort<T> {
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

impl<T> PortBase for InPort<T> {
	fn name(&self) -> ConstString {
		self.name.clone()
	}
}

impl<T> PortGetter<T> for InPort<T> {
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
}

impl<T> InPort<T> {
	#[must_use]
	pub fn new(name: impl Into<ConstString>) -> Self {
		Self {
			name: name.into(),
			src: RwLock::new(None),
		}
	}

	#[must_use]
	pub fn with_src(name: impl Into<ConstString>, src: impl Into<Arc<OutPort<T>>>) -> Self {
		Self {
			name: name.into(),
			src: RwLock::new(Some(src.into())),
		}
	}

	#[must_use]
	pub fn src(&self) -> Option<Arc<OutPort<T>>> {
		self.src.read().clone()
	}

	#[must_use]
	pub fn set_src(&self, src: impl Into<Arc<OutPort<T>>>) -> Option<Arc<OutPort<T>>> {
		self.src.write().replace(src.into())
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
		is_normal::<&InPort<i32>>();
		is_normal::<InPort<String>>();
	}
}
