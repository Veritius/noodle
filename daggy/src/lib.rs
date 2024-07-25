//! `noodle_core` [`Graph`](noodle_core::Graph) structures based on the `daggy` crate.

#![warn(missing_docs)]

mod cached;
mod edges;
mod id;
mod naive;

pub use cached::CachedGraph;
pub use naive::SimpleGraph;