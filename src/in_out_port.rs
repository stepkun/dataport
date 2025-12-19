// Copyright © 2025 Stephan Kunz
//! Port.

#![allow(unused)]

use core::{any::Any, ops::Deref};

use alloc::{boxed::Box, sync::Arc};

use crate::{Error, InPort, OutPort, PortBase, RwLock, RwLockReadGuard, any_extensions::AnySendSync};

/// InOutPort
/// Be aware, that the input and output side are connected.
/// If there is no OutPort value, the InPort value is used and consumed directly
pub struct InOutPort<T> {
	/// Internal [`OutPort`] which also provides an identifying name of the port,
	/// which must be unique for a given item.
	output: Arc<OutPort<T>>,
	/// Internal [`InPort`] which provides the same unique identifying name of
	/// the port as the internal [`OutPort`].
	input: Arc<InPort<T>>,
}

impl<T> core::fmt::Debug for InOutPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("InOutPort")
			.field("name", &self.output.name())
			//.field("src", &self.src)
			//.field("value", &self.out.value)
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

impl<T> InOutPort<T> {
	#[must_use]
	pub fn new(name: &'static str) -> Self {
		Self {
			output: Arc::new(OutPort::<T>::new(name)),
			input: Arc::new(InPort::<T>::new(name)),
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

	#[must_use]
	pub fn set_value(&self, value: impl Into<T>) -> Option<T> {
		self.output.set_value(value)
	}

	#[must_use]
	pub fn value(&self) -> Option<T> {
		// if the output has no value, use value from input
		let tmp = self.output.value();
		if tmp.is_none() {
			self.input.value().unwrap_or_default()
		} else {
			tmp
		}
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
