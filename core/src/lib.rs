#![cfg_attr(not(feature="std"), no_std)]

#![doc=include_str!("../../README.md")]
#![warn(missing_docs)]

extern crate alloc;

mod dirty;
mod group;
mod node;
mod socket;
mod solve;
mod graph;
mod value;

#[cfg(feature="petgraph")]
mod petgraph;

#[cfg(feature="daggy")]
mod daggy;

pub use dirty::*;
pub use group::*;
pub use node::*;
pub use socket::*;
pub use solve::*;
pub use graph::*;
pub use value::*;

#[cfg(feature="daggy")]
pub use daggy::*;