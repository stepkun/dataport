// Copyright © 2026 Stephan Kunz
//! Test [`PortVariant`]

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![allow(unused)]

use dataport::PortVariant;

#[test]
fn concept() {
	let mut inbound = PortVariant::create_inbound(42);
	let mut inoutbound = PortVariant::create_inoutbound(41);
	let mut outbound = PortVariant::create_outbound(40);
}
