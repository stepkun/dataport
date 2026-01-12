// Copyright © 2026 Stephan Kunz
//! Module containing 'flow' type ports.

pub mod in_out_port;
pub mod in_port;
pub mod out_port;

/// Trait for flow port types.
trait FlowCommons {}

/// Trait for incoming flow port types.
trait FlowIn: FlowCommons {}

/// Trait for incoming and outgoing flow port types.
trait FlowInOut: FlowIn + FlowOut {}

/// Trait for outgoing flow port types.
trait FlowOut: FlowCommons {}
