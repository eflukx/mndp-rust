//! MikroTik Neighbor Discovery Protocol (MNDP) library and discovery tool.
#![warn(missing_docs)]

mod neighbor;
mod protocol;

pub use crate::neighbor::{Builder, Neighbor, Unpack};
pub use crate::protocol::{MndpType, Packet, TypeValue};
