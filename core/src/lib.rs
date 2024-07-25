#![cfg_attr(not(feature="std"), no_std)]

#![doc=include_str!("../../README.md")]
#![warn(missing_docs)]

extern crate alloc;

mod graph;
mod group;
mod node;
mod socket;
mod value;

#[cfg(feature="petgraph")]
mod petgraph;

#[cfg(feature="daggy")]
mod daggy;

pub use graph::*;
pub use group::*;
pub use node::*;
pub use socket::*;
pub use value::*;

#[cfg(feature="daggy")]
pub use daggy::*;