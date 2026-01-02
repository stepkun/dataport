// Copyright © 2025 Stephan Kunz
//! Traits for working with ports and lists of ports.

use core::any::Any;

use crate::{
	ConstString,
	error::{Error, Result},
	in_out_port::InOutBoundPort,
	in_port::InBoundPort,
	port::Port,
	port_value::{PortValueReadGuard, PortValueWriteGuard},
};

/// The `AnyPort` trait allows to send ports between threads.
#[allow(unused)]
pub(crate) trait AnyPort: Any + Send + Sync + core::fmt::Debug + PortCommons {
	/// Convert to Any
	#[must_use]
	fn as_any(&self) -> &dyn Any;

	/// Convert to mut Any
	#[must_use]
	fn as_mut_any(&mut self) -> &mut dyn Any;
}

/// Blanket implementation for any type that has a `static` lifetime and implements
/// [`core::fmt::Debug`], [`PortCommons`], [`Send`] and [`Sync`].
impl<T: 'static + core::fmt::Debug + PortCommons + Send + Sync> AnyPort for T {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

/// Common features for all types of ports.
pub trait PortCommons {
	/// Returns an identifying name of the port.
	/// Must be unique within an item providing multiple ports.
	#[must_use]
	fn name(&self) -> ConstString;

	/// Returns the change sequence number, which wraps around to `1` after reaching u32::MAX.
	/// A sequence id of `0` means that the value has never been set or changed.
	fn sequence_number(&self) -> u32;
}

/// Trait for bound incoming port types.
pub trait InBound<T>: PortCommons {
	/// Returns a clone/copy of the T.
	#[must_use]
	fn get(&self) -> Option<T>
	where
		T: Clone;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn read(&self) -> Result<PortValueReadGuard<T>>;

	/// Returns an immutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_read(&self) -> Result<PortValueReadGuard<T>>;

	/// Returns the T, removing it from the port.
	#[must_use]
	fn take(&self) -> Option<T>;
}

/// Trait for bound combined in/out port types.
pub trait InOutBound<T> {
	/// Sets a new value to the T and returns the old T.
	#[must_use]
	fn replace(&self, value: impl Into<T>) -> Option<T>;
}

/// Trait for bound outgoing port types.
pub trait OutBound<T>: PortCommons {
	/// Sets a new value to the T.
	fn set(&self, value: impl Into<T>);

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn write(&self) -> Result<PortValueWriteGuard<T>>;

	/// Returns a mutable guard to the ports value T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_write(&self) -> Result<PortValueWriteGuard<T>>;
}

/// Something that provides ports.
pub trait PortProvider {
	/// Lookup a [`Port`].
	#[must_use]
	fn find(&self, name: impl Into<ConstString>) -> Option<&Port>;
}

/// Accessors to ports.
pub trait PortAccessors: PortProvider {
	/// Binds the in port to the out port.
	/// Port `out` is where the value is created, `in` where it is consumed.
	/// # Errors
	/// - [`Error::NotFound`], if one of the ports is not in port list.
	/// - [`Error::PortAlreadyBound`], if in port is already bound.
	/// - [`Error::WrongType`], if one of the ports is not the needed port type & type of T.
	fn bind_to<T: Any + Send + Sync>(
		&self,
		in_port: impl Into<ConstString>,
		out_list: &impl PortAccessors,
		out_port: impl Into<ConstString>,
	) -> Result<()> {
		// src is where the value is created, dest where it is consumed
		let src_port = out_port.into();
		if let Some(out_port) = out_list.find(src_port.clone()) {
			// src must provide an output value of the wanted type
			if let Some(out_value) = out_port.as_out_value::<T>() {
				let dest_port = in_port.into();
				if let Some(in_port) = self.find(dest_port.clone()) {
					// dest must want input value of the wanted type
					if let Some(input_port) = in_port.port().downcast_ref::<InBoundPort<T>>() {
						input_port.set_value(out_value);
						Ok(())
					} else if let Some(input_output_port) = in_port.port().downcast_ref::<InOutBoundPort<T>>() {
						input_output_port.set_value(out_value);
						Ok(())
					} else {
						Err(Error::WrongType { port: dest_port })
					}
				} else {
					Err(Error::NotFound { port: dest_port })
				}
			} else {
				Err(Error::WrongType { port: src_port })
			}
		} else {
			Err(Error::NotFound { port: src_port })
		}
	}

	/// Returns a copy of the value of that port.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn get<T: Any + Clone + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<T> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value) = port_ref.as_value::<T>() {
				if let Some(value) = value.read().get() {
					Ok(value)
				} else {
					Err(Error::ValueNotInitialized { port })
				}
			} else {
				Err(Error::NoValueSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns an immutable guard to the T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn read<T: Any + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortValueReadGuard<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				PortValueReadGuard::new(port_ref.name(), value_ref.clone())
			} else {
				Err(Error::NoValueSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns an immutable guard to the T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_read<T: Any + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortValueReadGuard<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				PortValueReadGuard::try_new(port_ref.name(), value_ref.clone())
			} else {
				Err(Error::NoValueSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Replaces the port's value with the `value` and returns the old value.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn replace<T: Any + Send + Sync>(&self, port: &str, value: impl Into<T>) -> Result<Option<T>> {
		if let Some(port_ref) = self.find(port) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				Ok(value_ref.write().replace(value))
			} else {
				Err(Error::NoValueSet { port: port.into() })
			}
		} else {
			Err(Error::NotFound { port: port.into() })
		}
	}

	/// Returns the sequence number of the [`Port`]s value.
	/// # Errors
	/// - [`Error::NotFound`] if `port` is not contained.
	fn sequence_number(&self, port: impl Into<ConstString>) -> Result<u32> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			Ok(port_ref.sequence_number())
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Sets the port to the value.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn set<T: Any + Send + Sync>(&self, port: impl Into<ConstString>, value: impl Into<T>) -> Result<()> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				value_ref.write().set(value);
				Ok(())
			} else {
				Err(Error::NoValueSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns the value of that port.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn take<T: Any + Clone + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<Option<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				Ok(value_ref.write().take())
			} else {
				Err(Error::NoValueSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns a mutable guard to the T.
	/// # Errors
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn write<T: Any + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortValueWriteGuard<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				PortValueWriteGuard::new(port_ref.name(), value_ref.clone())
			} else {
				Err(Error::NoValueSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}

	/// Returns a mutable guard to the T.
	/// # Errors
	/// - [`Error::IsLocked`], if port is locked.
	/// - [`Error::NotFound`], if port is not in port list.
	/// - [`Error::WrongType`], if port is not the expected port type & type of T.
	fn try_write<T: Any + Send + Sync>(&self, port: impl Into<ConstString>) -> Result<PortValueWriteGuard<T>> {
		let port = port.into();
		if let Some(port_ref) = self.find(port.clone()) {
			// port must have a value of the wanted type
			if let Some(value_ref) = port_ref.as_value::<T>() {
				PortValueWriteGuard::try_new(port_ref.name(), value_ref.clone())
			} else {
				Err(Error::NoValueSet { port })
			}
		} else {
			Err(Error::NotFound { port })
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{in_port::InBoundPort, out_port::OutBoundPort};

	use super::*;

	fn return_impl_in_port() -> impl InBound<i32> {
		InBoundPort::new("port")
	}

	fn use_impl_in_port(src: impl InBound<i32>) {
		assert!(src.read().is_err());
		assert!(src.get().is_none());
		assert!(src.take().is_none());
	}

	fn return_impl_inout_port() -> impl InOutBound<i32> {
		InOutBoundPort::new("port")
	}

	fn use_impl_inout_port(src: impl InOutBound<i32>) {
		assert!(src.replace(24).is_none());
		assert_eq!(src.replace(42).unwrap(), 24);
	}

	fn return_impl_out_port() -> impl OutBound<i32> {
		OutBoundPort::new("port")
	}

	fn use_impl_out_port(src: impl OutBound<i32>) {
		src.set(22);
		*src.write().unwrap() = 24;
	}

	#[test]
	fn impl_compatibility() {
		let in_port = return_impl_in_port();
		use_impl_in_port(in_port);

		let out_port = return_impl_out_port();
		use_impl_out_port(out_port);

		let inout_port = return_impl_inout_port();
		use_impl_inout_port(inout_port);
	}

	//#[test]
	//fn dyn_compatibility() {
	//	let ip: Arc<dyn InPort<i32>> = InputPort::new("in_port");
	//	let op: Arc<dyn OutPort<i32>> = OutputPort::new("out_port");
	//}
}
