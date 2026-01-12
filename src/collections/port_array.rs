// Copyright © 2026 Stephan Kunz
//! A fixed unsorted collection of ports.

use crate::{
	ConstString,
	collections::{PortAccessors, PortProvider},
	port_variant::PortVariant,
};

/// A fixed unsorted array of [`PortVariant`]s.
#[repr(transparent)]
pub struct PortArray<const S: usize>([(ConstString, PortVariant); S]);

impl<const S: usize> PortArray<S> {
	pub fn new(ports: [(ConstString, PortVariant); S]) -> Self {
		Self(ports)
	}
}

impl<const S: usize> core::fmt::Debug for PortArray<S> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("PortArray").field(&self.0).finish()
	}
}

impl<const S: usize> core::ops::Deref for PortArray<S> {
	type Target = [(ConstString, PortVariant)];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<const S: usize> PortProvider for PortArray<S> {
	fn contains_key(&self, name: &str) -> bool {
		self.0.iter().any(|port| &*port.0 == name)
	}

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

impl<const S: usize> PortAccessors for PortArray<S> {}

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
