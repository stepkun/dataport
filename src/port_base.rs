// Copyright © 2025 Stephan Kunz
//! Implementation of the [`Portbase`].

#![allow(unused)]

use alloc::collections::btree_map::BTreeMap;

use crate::{
	ConstString, Error, InPort, InputOutputPort, OutPort, Port, PortCommons, PortValueReadGuard, PortValueWriteGuard,
	error::Result,
};

/// Holds all [`Databoard`](crate::databoard::Databoard) data.
#[derive(Default)]
pub struct PortBase {
	storage: BTreeMap<ConstString, Port>,
}

impl PortBase {
	/// Returns `true` if a [`Port`] with name `key` is available, otherwise `false`.
	#[must_use]
	pub fn contains_key(&self, key: &str) -> bool {
		self.storage.contains_key(key)
	}

	/// Returns  a result of `true` if a [`Port`] under`key` of type `T` is available, otherwise a result of `false`.
	/// # Errors
	/// - [`Error::WrongType`] if the [`Port`] has not the expected type `T`.
	pub fn contains<T: 'static + Send + Sync>(&self, key: &str) -> Result<bool> {
		self.storage.get(key).map_or(Ok(false), |port| {
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
		if self.storage.contains_key(&key) {
			return Err(Error::AlreadyExists { port: key });
		}
		let iop = InputOutputPort::<T>::with_value(key.clone(), value);
		let port = Port::from(iop);
		self.storage.insert(key, port);
		Ok(())
	}

	/// Returns the value of type `T` stored in the [`Port`] under `key` and deletes it from storage.
	/// # Errors
	/// - [`Error::NotFound`]  if `key` is not contained.
	/// - [`Error::WrongType`] if the entry has not the expected type `T`.
	pub fn delete<T: 'static + Send + Sync>(&mut self, key: &str) -> Result<T> {
		match self.contains::<T>(key) {
			Ok(_) => self.storage.remove(key).map_or_else(
				|| Err(Error::NotFound { port: key.into() }),
				|port| {
					port.as_in_out_port::<T>().map_or_else(
						|| Err(Error::WrongType { port: key.into() }),
						|port| {
							port.take()
								.map_or_else(|| Err(Error::ValueNotSet { port: key.into() }), |value| Ok(value))
						},
					)
				},
			),
			Err(err) => Err(err),
		}
	}

	/// Returns a copy of the value of type `T` stored in the [`Port`] under `key`.
	/// # Errors
	/// - [`Error::NotFound`]  if `key` is not contained.
	/// - [`Error::WrongType`] if the entry has not the expected type `T`.
	pub fn get<T: 'static + Clone + Send + Sync>(&self, key: &str) -> Result<Option<T>> {
		self.storage.get(key).map_or_else(
			|| Err(Error::NotFound { port: key.into() }),
			|port| {
				port.as_in_out_port::<T>()
					.map_or_else(|| Err(Error::WrongType { port: key.into() }), |port| Ok(port.get()))
			},
		)
	}

	/// Returns a clone of the [`Port`]
	/// # Errors
	/// - [`Error::NotFound`] if `key` is not contained.
	pub fn port(&self, key: &str) -> Result<Port> {
		self.storage
			.get(key)
			.map_or_else(|| Err(Error::NotFound { port: key.into() }), |port| Ok(port.clone()))
	}

	/// Returns a read guard to the `T` of the [`Port`] stored under `key`.
	/// The [`Port`] is locked for write while this guard is held.
	///
	/// You need to drop the received [`PortReadGuard`] before using any write operation.
	/// # Errors
	/// - [`Error::NotFound`]  if `key` is not contained.
	/// - [`Error::WrongType`] if the port has not the expected type `T`.
	pub fn read<T: 'static + Send + Sync>(&self, key: &str) -> Result<PortValueReadGuard<T>> {
		self.storage.get(key).map_or_else(
			|| Err(Error::NotFound { port: key.into() }),
			|port| {
				port.as_in_out_port::<T>().map_or_else(
					|| Err(Error::WrongType { port: key.into() }),
					|port| port.read().map_or_else(Err, Ok),
				)
			},
		)
	}

	/// Returns a read guard to the `T` of the [`Port`] stored under `key`.
	/// The [`Port`] is locked for write while this guard is held.
	///
	/// You need to drop the received [`PortReadGuard`] before using any write operation.
	/// # Errors
	/// - [`Error::IsLocked`]  if the [`Port`] is locked by someone else.
	/// - [`Error::NotFound`]  if `key` is not contained.
	/// - [`Error::WrongType`] if the [`Port`] has not the expected type `T`.
	pub fn try_read<T: 'static + Send + Sync>(&self, key: &str) -> Result<PortValueReadGuard<T>> {
		self.storage.get(key).map_or_else(
			|| Err(Error::NotFound { port: key.into() }),
			|port| {
				port.as_in_out_port::<T>().map_or_else(
					|| Err(Error::WrongType { port: key.into() }),
					|port| port.try_read().map_or_else(Err, Ok),
				)
			},
		)
	}

	/// Returns the sequence number of the [`Port`]s value.
	/// # Errors
	/// - [`Error::NotFound`] if `key` is not contained.
	pub fn sequence_number(&self, key: &str) -> Result<u32> {
		self.storage.get(key).map_or_else(
			|| Err(Error::NotFound { port: key.into() }),
			|port| Ok(port.sequence_number()),
		)
	}

	/// Updates a value of type `T` stored under `key` and returns the old value.
	/// # Errors
	/// - [`Error::NotFound`]  if `key` is not contained.
	/// - [`Error::WrongType`] if the [`Port`] has not the expected type `T`.
	pub fn update<T: 'static + Send + Sync>(&self, key: &str, value: impl Into<T>) -> Result<Option<T>> {
		self.storage.get(key).map_or_else(
			|| Err(Error::NotFound { port: key.into() }),
			|port| {
				port.as_in_out_port::<T>().map_or_else(
					|| Err(Error::WrongType { port: key.into() }),
					|port| Ok(port.replace(value.into())),
				)
			},
		)
	}

	/// Returns a write guard to the `T` of the [`Port`] stored under `key`.
	/// The [`Port`] is locked for read & write while this guard is held.
	/// Multiple changes during holding the guard are counted as a single change,
	/// so `sequence_id()`will only increase by 1.
	///
	/// You need to drop the received [`PortWriteGuard`] before using any other operation.
	/// # Errors
	/// - [`Error::NotFound`]  if `key` is not contained.
	/// - [`Error::WrongType`] if the entry has not the expected type `T`.
	pub fn write<T: 'static + Send + Sync>(&self, key: &str) -> Result<PortValueWriteGuard<T>> {
		self.storage.get(key).map_or_else(
			|| Err(Error::NotFound { port: key.into() }),
			|port| {
				port.as_in_out_port::<T>().map_or_else(
					|| Err(Error::WrongType { port: key.into() }),
					|port| port.write().map_or_else(Err, Ok),
				)
			},
		)
	}

	/// Returns a write guard to the `T` of the [`Port`] stored under `key`.
	/// The [`Port`] is locked for read & write while this guard is held.
	/// Multiple changes during holding the guard are counted as a single change,
	/// so `sequence_id()`will only increase by 1.
	///
	/// You need to drop the received [`PortWriteGuard`] before using any other operation.
	/// # Errors
	/// - [`Error::IsLocked`]  if the [`Port`] is locked by someone else.
	/// - [`Error::NotFound`]  if `key` is not contained.
	/// - [`Error::WrongType`] if the [`Port`] has not the expected type `T`.
	pub fn try_write<T: 'static + Send + Sync>(&self, key: &str) -> Result<PortValueWriteGuard<T>> {
		self.storage.get(key).map_or_else(
			|| Err(Error::NotFound { port: key.into() }),
			|port| {
				port.as_in_out_port::<T>().map_or_else(
					|| Err(Error::WrongType { port: key.into() }),
					|port| port.try_write().map_or_else(Err, Ok),
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
		is_normal::<PortBase>();
	}
}
