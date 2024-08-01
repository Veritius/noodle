use crate::*;

/// A type that can produce new [`Node`] types.
pub trait NodeFactory {
    /// The output type of the node.
    type Output: Node;

    /// Creates a new instance of the node with default configuration.
    fn new_default(&self) -> Self::Output;
}