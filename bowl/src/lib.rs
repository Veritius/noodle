//! A collection of highly generic types for `noodle_core`.

#![cfg_attr(not(feature="std"), no_std)]

#![warn(missing_docs)]

// Dependency stuff
extern crate alloc;

// Re-exports
pub use noodle_core;

// Internal modules
mod linked;

pub mod internals {
    //! Internal types.

    pub use crate::linked::{HashGraph, VertexItem, EdgeSet};
}