// Copyright © 2026 Stephan Kunz
//! Abstract port value type trait.

use core::any::Any;

/// The `AnyPortValueType` trait allows to use different types of values in ports.
pub trait AnyPortValueType: Any + Send + Sync + core::fmt::Debug {
	/// Convert to Any
	#[must_use]
	fn as_any(&self) -> &dyn Any;

	/// Convert to mut Any
	#[must_use]
	fn as_mut_any(&mut self) -> &mut dyn Any;
}

/// Blanket implementation for any type that implements
/// [`Any`], [`Send`], [`Sync`] and [`core::fmt::Debug`].
impl<T: Any + Send + Sync + core::fmt::Debug> AnyPortValueType for T {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_mut_any(&mut self) -> &mut dyn Any {
		self
	}
}
