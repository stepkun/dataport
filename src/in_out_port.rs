// Copyright © 2025 Stephan Kunz
//! Implementation of a port providing both [`InPort`] and [`OutPort`].

use core::any::Any;

use alloc::sync::Arc;

use crate::{
	ConstString,
	any_port::AnyPort,
	error::Result,
	guards::{PortReadGuard, PortWriteGuard},
	in_port::InputPort,
	out_port::OutputPort,
	traits::{InPort, OutPort, PortBase},
};

/// InputOutputPort
/// Be aware, that the input and output side are not automatically connected.
/// The input value has to be propagated manually.
pub struct InputOutputPort<T> {
	/// Internal [`InPort`] which also provides an identifying name of the port,
	/// which must be unique for a given item.
	input: Arc<InputPort<T>>,
	/// Internal [`OutPort`] which provides the same unique identifying name of
	/// the port as the internal [`InPort`].
	output: Arc<OutputPort<T>>,
}

impl<T: 'static + Send + Sync> AnyPort for InputOutputPort<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

impl<T> core::fmt::Debug for InputOutputPort<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("InputOutputPort")
			.field("name", &self.input.name())
			//.field("input", &self.input)
			//.field("output", &self.output)
			.finish_non_exhaustive()
	}
}

impl<T: 'static> PartialEq for InputOutputPort<T> {
	/// Partial equality of an in/out port is, if input and output parts are partial equal
	fn eq(&self, other: &Self) -> bool {
		self.input == other.input && self.output == other.output
	}
}

impl<T> PortBase for InputOutputPort<T> {
	fn name(&self) -> ConstString {
		self.output.name()
	}
}

impl<T> InPort<T> for InputOutputPort<T> {
	fn get(&self) -> Option<T>
	where
		T: Clone,
	{
		self.input.get()
	}

	fn read(&self) -> Result<PortReadGuard<T>> {
		InPort::read(&*self.input)
	}

	fn sequence_id(&self) -> Option<u32> {
		self.input.sequence_id()
	}

	fn try_read(&self) -> Result<PortReadGuard<T>> {
		InPort::try_read(&*self.input)
	}

	fn take(&self) -> Option<T> {
		self.input.take()
	}

	fn src(&self) -> Option<Arc<OutputPort<T>>> {
		self.input.src()
	}

	fn replace_src(&self, src: impl Into<Arc<OutputPort<T>>>) -> Option<Arc<OutputPort<T>>> {
		self.input.replace_src(src)
	}
}

impl<T> OutPort<T> for InputOutputPort<T> {
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

impl<T> InputOutputPort<T> {
	#[must_use]
	pub fn new(name: impl Into<ConstString>) -> Self {
		let name = name.into();
		Self {
			input: Arc::new(InputPort::<T>::new(name.clone())),
			output: Arc::new(OutputPort::<T>::new(name)),
		}
	}

	#[must_use]
	pub fn with_src(name: impl Into<ConstString>, src: impl Into<Arc<OutputPort<T>>>) -> Self {
		let name = name.into();
		Self {
			input: Arc::new(InputPort::<T>::with_src(name.clone(), src)),
			output: Arc::new(OutputPort::<T>::new(name)),
		}
	}

	#[must_use]
	pub fn with_value(name: impl Into<ConstString>, value: impl Into<T>) -> Self {
		let name = name.into();
		Self {
			input: Arc::new(InputPort::<T>::new(name.clone())),
			output: Arc::new(OutputPort::<T>::with_value(name, value)),
		}
	}

	/// Propagate an eventually existing value from input to output.
	pub fn propagate(&self) {
		if let Some(src) = self.src()
			&& let Some(value) = src.by_value()
		{
			self.output.set(value);
		};
	}

	pub fn input(&self) -> Arc<InputPort<T>> {
		self.input.clone()
	}

	pub fn output(&self) -> Arc<OutputPort<T>> {
		self.output.clone()
	}
}

/// Automatic conversion from InOutPort to InPort
impl<T> From<InputOutputPort<T>> for Arc<InputPort<T>> {
	fn from(value: InputOutputPort<T>) -> Self {
		value.input.clone()
	}
}

/// Automatic conversion from InOutPort to InPort
impl<T> From<InputOutputPort<T>> for Arc<OutputPort<T>> {
	fn from(value: InputOutputPort<T>) -> Self {
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
		is_normal::<&InputOutputPort<f32>>();
		is_normal::<InputOutputPort<String>>();
	}
}
