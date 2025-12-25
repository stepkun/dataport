// Copyright © 2025 Stephan Kunz
//! Provides extensions to [`Any`]

#![allow(unused)]

use core::any::Any;

use crate::traits::PortBase;

/// The `AnySendSync` trait allows to send data between threads.
pub(crate) trait AnySendSync: Any + Send + Sync {
	/// Convert to Any
	#[must_use]
	fn as_any(&self) -> &dyn Any;

	/// Convert to mut Any
	#[must_use]
	fn as_mut_any(&mut self) -> &mut dyn Any;
}

/// Implementation for any type that has a `static` lifetime and implements [`Send`] and [`Sync`].
impl<T: 'static + Send + Sync> AnySendSync for T {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}

pub(crate) trait AnyPort: AnySendSync + core::fmt::Debug + PortBase {
	/// Convert to Any
	#[must_use]
	fn as_any(&self) -> &dyn Any;

	/// Convert to mut Any
	#[must_use]
	fn as_mut_any(&mut self) -> &mut dyn Any;
}
