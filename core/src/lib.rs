#![cfg_attr(not(feature="std"), no_std)]

#![doc=include_str!("../../README.md")]
#![warn(missing_docs)]

extern crate alloc;

mod group;
mod node;
mod socket;
mod graph;

#[cfg(feature="petgraph")]
mod petgraph;

#[cfg(feature="daggy")]
mod daggy;

pub use group::*;
pub use node::*;
pub use socket::*;
pub use graph::*;

#[cfg(feature="daggy")]
pub use daggy::*;