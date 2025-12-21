// Copyright © 2025 Stephan Kunz
//! Port.

#![allow(unused)]

use core::{any::Any, ops::Deref};

use alloc::{boxed::Box, sync::Arc};

use crate::{
	Error, InPort, OutPort, PortBase, PortGetter, PortReadGuard, PortSetter, PortWriteGuard, Result, RwLock,
	any_extensions::AnySendSync,
};

/// InOutPort
/// Be aware, that the input and output side are not automatically connected.
/// The input value has to be propagated manually.
pub struct InOutPort<T> {
	/// Internal [`InPort`] which also provides an identifying name of the port,
	/// which must be unique for a given item.
	input: Arc<InPort<T>>,
	/// Internal [`OutPort`] which provides the same unique identifying name of
	/// the port as the internal [`InPort`].
	output: Arc<OutPort<T>>,
}

impl<T> core::fmt::Debug for InOutPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("InOutPort")
			.field("name", &self.input.name())
			//.field("input", &self.input)
			//.field("output", &self.output)
			.finish_non_exhaustive()
	}
}

impl<T: 'static> PartialEq for InOutPort<T> {
	/// Partial equality of an in/out port is, if input and output parts are partial equal
	fn eq(&self, other: &Self) -> bool {
		self.input == other.input && self.output == other.output
	}
}

impl<T> PortBase for InOutPort<T> {
	fn name(&self) -> &'static str {
		self.output.name()
	}
}

impl<T> PortGetter<T> for InOutPort<T> {
	fn as_ref(&self) -> Result<PortReadGuard<T>> {
		self.output.by_ref()
	}

	fn get(&self) -> Option<T>
	where
		T: Clone,
	{
		self.output.by_copy()
	}

	fn take(&self) -> Option<T> {
		self.output.by_value()
	}
}

impl<T> PortSetter<T> for InOutPort<T> {
	fn as_mut(&self) -> Result<PortWriteGuard<T>> {
		self.output.as_mut()
	}

	fn set(&self, value: impl Into<T>) -> Option<T> {
		self.output.set(value)
	}
}

impl<T> InOutPort<T> {
	#[must_use]
	pub fn new(name: &'static str) -> Self {
		Self {
			input: Arc::new(InPort::<T>::new(name)),
			output: Arc::new(OutPort::<T>::new(name)),
		}
	}

	#[must_use]
	pub fn with(name: &'static str, src: impl Into<Arc<OutPort<T>>>) -> Self {
		Self {
			input: Arc::new(InPort::<T>::with(name, src)),
			output: Arc::new(OutPort::<T>::new(name)),
		}
	}

	#[must_use]
	pub fn src(&self) -> Option<Arc<OutPort<T>>> {
		self.input.src()
	}

	#[must_use]
	pub fn set_src(&self, src: impl Into<Arc<OutPort<T>>>) -> Option<Arc<OutPort<T>>> {
		self.input.set_src(src)
	}

	/// Propagate an evantually existing value from input to output.
	pub fn propagate(&self) {
		if let Some(value) = self.src().unwrap().by_value() {
			let _x = self.output.set(value);
		};
	}
}

/// Automatic conversion from InOutPort to InPort
impl<T> From<InOutPort<T>> for Arc<InPort<T>> {
	fn from(value: InOutPort<T>) -> Self {
		value.input.clone()
	}
}

/// Automatic conversion from InOutPort to InPort
impl<T> From<InOutPort<T>> for Arc<OutPort<T>> {
	fn from(value: InOutPort<T>) -> Self {
		value.output.clone()
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
		is_normal::<&InOutPort<f32>>();
		is_normal::<InOutPort<String>>();
	}
}
