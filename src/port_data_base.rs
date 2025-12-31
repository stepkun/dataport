// Copyright © 2025 Stephan Kunz
//! Implementation of a [`PortDataBase`].

use alloc::collections::btree_map::BTreeMap;

use crate::{
	ConstString, PortAccessors, PortProvider,
	error::{Error, Result},
	in_out_port::InputOutputPort,
	port::Port,
	traits::{InOutPort, InPort, PortCommons},
};

/// A database like container for [`Port`]s.
#[derive(Default)]
#[repr(transparent)]
pub struct PortDataBase(BTreeMap<ConstString, Port>);

impl core::fmt::Debug for PortDataBase {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("PortDataBase")
			.field(&self.0)
			.finish()
	}
}

impl PortProvider for PortDataBase {
	fn find(&self, name: impl Into<ConstString>) -> Option<&Port> {
		let name = name.into();
		self.0
			.values()
			.into_iter()
			.find(|&port| port.name() == name)
			.map(|v| v as _)
	}
}

impl PortAccessors for PortDataBase {}

impl PortDataBase {
	/// Returns `true` if a [`Port`] with name `key` is available, otherwise `false`.
	#[must_use]
	pub fn contains_key(&self, key: &str) -> bool {
		self.0.contains_key(key)
	}

	/// Returns  a result of `true` if a [`Port`] under`key` of type `T` is available, otherwise a result of `false`.
	/// # Errors
	/// - [`Error::WrongType`] if the [`Port`] has not the expected type `T`.
	pub fn contains<T: 'static + Send + Sync>(&self, key: &str) -> Result<bool> {
		self.0.get(key).map_or(Ok(false), |port| {
			port.as_in_out_port::<T>().map_or_else(
				|| Err(Error::WrongType { port: key.into() }),
				|port| if port.name().as_ref() == key { Ok(true) } else { Ok(false) },
			)
		})
	}

	/// Creates a [`Port`] with value of type `T` under `key`.
	/// # Errors
	/// - [`Error::AlreadyExists`] if `key` already exists.
	pub fn create<T: 'static + Send + Sync>(&mut self, key: impl Into<ConstString>, value: impl Into<T>) -> Result<()> {
		let key = key.into();
		if self.0.contains_key(&key) {
			return Err(Error::AlreadyExists { port: key });
		}
		let iop = InputOutputPort::<T>::with_value(key.clone(), value);
		let port = Port::from(iop);
		self.0.insert(key, port);
		Ok(())
	}

	/// Returns the value of type `T` stored in the [`Port`] under `key` and deletes it from storage.
	/// # Errors
	/// - [`Error::NotFound`]  if `key` is not contained.
	/// - [`Error::WrongType`] if the entry has not the expected type `T`.
	pub fn delete<T: 'static + Send + Sync>(&mut self, key: &str) -> Result<T> {
		match self.contains::<T>(key) {
			Ok(_) => self.0.remove(key).map_or_else(
				|| Err(Error::NotFound { port: key.into() }),
				|port| {
					port.as_in_out_port::<T>().map_or_else(
						|| Err(Error::WrongType { port: key.into() }),
						|port| {
							port.take()
								.map_or_else(|| Err(Error::NoValueSet { port: key.into() }), |value| Ok(value))
						},
					)
				},
			),
			Err(err) => Err(err),
		}
	}

	/// Returns a reference to the [`Port`]
	/// # Errors
	/// - [`Error::NotFound`] if `key` is not contained.
	pub fn port(&self, key: &str) -> Result<Port> {
		self.0
			.get(key)
			.map_or_else(|| Err(Error::NotFound { port: key.into() }), |port| Ok(port.clone()))
	}

	/// Updates a value of type `T` stored under `key` and returns the old value.
	/// # Errors
	/// - [`Error::NotFound`]  if `key` is not contained.
	/// - [`Error::WrongType`] if the [`Port`] has not the expected type `T`.
	pub fn update<T: 'static + Send + Sync>(&self, key: &str, value: impl Into<T>) -> Result<Option<T>> {
		self.0.get(key).map_or_else(
			|| Err(Error::NotFound { port: key.into() }),
			|port| {
				port.as_in_out_port::<T>().map_or_else(
					|| Err(Error::WrongType { port: key.into() }),
					|port| Ok(port.replace(value.into())),
				)
			},
		)
	}
}

#[cfg(test)]
mod tests {
	#![allow(clippy::unwrap_used)]
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<PortDataBase>();
	}
}
