// Copyright © 2025 Stephan Kunz
//! Port.

use core::any::Any;

use alloc::sync::Arc;

use crate::{
	ConstString,
	any_port::AnyPort,
	error::Result,
	guards::{PortReadGuard, PortWriteGuard},
	in_port::InPort,
	out_port::OutPort,
	traits::{PortBase, PortGetter, PortSetter},
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

impl<T: 'static + Send + Sync> AnyPort for InOutPort<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
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
	fn name(&self) -> ConstString {
		self.output.name()
	}
}

impl<T> PortGetter<T> for InOutPort<T> {
	fn get(&self) -> Option<T>
	where
		T: Clone,
	{
		self.input.get()
	}

	fn read(&self) -> Result<PortReadGuard<T>> {
		PortGetter::read(&*self.input)
	}

	fn try_read(&self) -> Result<PortReadGuard<T>> {
		PortGetter::try_read(&*self.input)
	}

	fn take(&self) -> Option<T> {
		self.input.take()
	}
}

impl<T> PortSetter<T> for InOutPort<T> {
	fn replace(&self, value: impl Into<T>) -> Option<T> {
		self.output.replace(value)
	}

	fn set(&self, value: impl Into<T>) {
		self.output.set(value)
	}

	fn write(&self) -> Result<PortWriteGuard<T>> {
		self.output.write()
	}

	fn try_write(&self) -> Result<PortWriteGuard<T>> {
		self.output.try_write()
	}
}

impl<T> InOutPort<T> {
	#[must_use]
	pub fn new(name: impl Into<ConstString>) -> Self {
		let name = name.into();
		Self {
			input: Arc::new(InPort::<T>::new(name.clone())),
			output: Arc::new(OutPort::<T>::new(name)),
		}
	}

	#[must_use]
	pub fn with_src(name: impl Into<ConstString>, src: impl Into<Arc<OutPort<T>>>) -> Self {
		let name = name.into();
		Self {
			input: Arc::new(InPort::<T>::with_src(name.clone(), src)),
			output: Arc::new(OutPort::<T>::new(name)),
		}
	}

	#[must_use]
	pub fn with_value(name: impl Into<ConstString>, value: impl Into<T>) -> Self {
		let name = name.into();
		Self {
			input: Arc::new(InPort::<T>::new(name.clone())),
			output: Arc::new(OutPort::<T>::with_value(name, value)),
		}
	}

	#[must_use]
	pub fn dest(&self) -> Arc<OutPort<T>> {
		self.output.clone()
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
		if let Some(src) = self.src()
			&& let Some(value) = src.by_value()
		{
			self.output.set(value);
		};
	}

	pub(crate) fn input(&self) -> Arc<InPort<T>> {
		self.input.clone()
	}

	pub(crate) fn output(&self) -> Arc<OutPort<T>> {
		self.output.clone()
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
	use alloc::string::String;

	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&InOutPort<f32>>();
		is_normal::<InOutPort<String>>();
	}
}
