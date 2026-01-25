// Copyright © 2026 Stephan Kunz
//! Test [`PortVariant`]

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::PortVariant;

#[test]
fn sequence_number() {
	let mut inbound = PortVariant::create_inbound(42);
	assert_eq!(inbound.sequence_number(), 1);
	let mut inoutbound = PortVariant::create_inoutbound(41);
	assert_eq!(inoutbound.sequence_number(), 1);
	assert!(inoutbound.set(42).is_ok());
	assert_eq!(inoutbound.sequence_number(), 2);
	let mut outbound = PortVariant::create_outbound(40);
	assert_eq!(outbound.sequence_number(), 1);
	assert!(outbound.set(41).is_ok());
	assert_eq!(outbound.sequence_number(), 2);
}
