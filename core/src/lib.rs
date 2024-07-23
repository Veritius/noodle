#![cfg_attr(not(feature="std"), no_std)]

#![doc=include_str!("../../README.md")]
#![warn(missing_docs)]

extern crate alloc;

#[cfg(feature="petgraph")]
mod petgraph;

mod dirty;
mod group;
mod node;
mod socket;
mod solve;
mod store;
mod value;

pub use dirty::*;
pub use group::*;
pub use node::*;
pub use socket::*;
pub use solve::*;
pub use store::*;
pub use value::*;