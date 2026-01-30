// Copyright © 2026 Stephan Kunz
//! Most commonly used interfaces of [`dataport`](crate).
//!
//! Typically it is sufficient to include the prelude with
//!
//! ```use databoarde::prelude::*;```

pub use crate::{
	PortArray, PortCollection, PortCollectionAccessors, PortCollectionAccessorsMut, PortCollectionMut, create_inbound_entry,
	create_inoutbound_entry, create_outbound_entry,
};
