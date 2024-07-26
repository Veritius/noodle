#![cfg_attr(not(feature="std"), no_std)]

#![doc=include_str!("../../README.md")]
#![warn(missing_docs)]

extern crate alloc;

mod graph;
mod node;
mod socket;
mod value;

pub use graph::*;
pub use node::*;
pub use socket::*;
pub use value::*;