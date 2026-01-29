// Copyright © 2025 Stephan Kunz
//! A special [`SequenceNumber`].

/// A sequence number which
/// - starts at `0`,
/// - can only be incremeted by 1 and
/// - wraps around to `1` when exceeding its limits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct SequenceNumber(u32);

impl SequenceNumber {
	pub(crate) fn increment(&mut self) {
		if self.0 < u32::MAX {
			self.0 += 1;
		} else {
			self.0 = 1;
		}
	}

	pub const fn value(&self) -> u32 {
		self.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const fn is_normal<T: Sized + Send + Sync>() {}

	// check, that the auto traits are available.
	#[test]
	const fn normal_types() {
		is_normal::<&SequenceNumber>();
		is_normal::<SequenceNumber>();
	}

	#[test]
	fn creation() {
		let mut sq = SequenceNumber::default();
		assert_eq!(sq.value(), 0);
		sq.increment();
		assert_eq!(sq.value(), 1);
		sq.0 = u32::MAX - 1;
		sq.increment();
		assert_eq!(sq.value(), u32::MAX);
		sq.increment();
		assert_eq!(sq.value(), 1);
	}
}
