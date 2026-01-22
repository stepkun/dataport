// Copyright © 2026 Stephan Kunz
//! A fixed unsorted collection of ports.

use core::ops::Deref;

use crate::{ConstString, collections::PortCollection, port_variant::PortVariant};

pub static EMPTY_PORT_ARRAY: PortArray<0> = PortArray([]);

/// A fixed unsorted array of [`PortVariant`]s.
#[repr(transparent)]
pub struct PortArray<const S: usize>([(ConstString, PortVariant); S]);

impl<const S: usize> PortArray<S> {
	pub fn from(ports: [(ConstString, PortVariant); S]) -> Self {
		Self(ports)
	}
}

impl PortArray<0> {
	pub fn empty() -> Self {
		Self::from([])
	}
}

impl<const S: usize> core::fmt::Debug for PortArray<S> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("PortArray").field(&self.0).finish()
	}
}

impl<const S: usize> Deref for PortArray<S> {
	type Target = [(ConstString, PortVariant)];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<const S: usize> PortCollection for PortArray<S> {
	fn find(&self, name: &str) -> Option<&PortVariant> {
		self.0
			.iter()
			.find(|port| &*port.0 == name)
			.map(|v| &v.1 as _)
	}

	fn find_mut(&mut self, name: &str) -> Option<&mut PortVariant> {
		self.0
			.iter_mut()
			.find(|port| &*port.0 == name)
			.map(|v| &mut v.1 as _)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&PortArray<2>>();
		is_normal::<PortArray<4>>();
	}
}
