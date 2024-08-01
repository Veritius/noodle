//! A collection of highly generic types for `noodle_core`.

#![cfg_attr(not(feature="std"), no_std)]

#![warn(missing_docs)]

// Dependency stuff
extern crate alloc;

// Re-exports
pub use noodle_core;

mod link;
mod vertex;

// Public modules
pub mod hashgraph;

// Public exports
pub use hashgraph::HashGraph;
pub use link::{Link, Links};
pub use vertex::Vertex;