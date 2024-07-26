use crate::*;

/// Returned when adding a link would form a cycle in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WouldCycle;

impl core::fmt::Display for WouldCycle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("would cycle")
    }
}

#[cfg(feature="std")]
impl std::error::Error for WouldCycle {}

/// An error that occurred while trying to execute a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeExecutionError {
    /// Returned when a node does not have the sufficient inputs to execute it.
    NotEnoughInputs,

    /// Returned when an [`OutputMask`] masked out an output that must be used.
    MustUseOutput {
        /// The ID of the socket that was masked out.
        socket: SocketId,
    },
}

impl core::fmt::Display for NodeExecutionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NodeExecutionError::NotEnoughInputs => write!(f, "not enough inputs"),
            NodeExecutionError::MustUseOutput { socket } => write!(f, "output must be used: {socket:?}"),
        }
    }
}

#[cfg(feature="std")]
impl std::error::Error for NodeExecutionError {}

/// An error that occurred while trying to solve a graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphSolveError {
    /// A directed cycle was detected, and processing was aborted.
    DirectedCycleOccurred,

    /// An individual node in the processing chain failed.
    NodeExecutionFailed {
        /// The ID of the node that failed.
        node: NodeId,

        /// The error returned on failure.
        error: NodeExecutionError,
    },
}

impl core::fmt::Display for GraphSolveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GraphSolveError::DirectedCycleOccurred => write!(f, "directed cycle detected"),
            GraphSolveError::NodeExecutionFailed { node, error } => write!(f, "node {node:?} failed to execute: {error:?}"),
        }
    }
}

#[cfg(feature="std")]
impl std::error::Error for GraphSolveError {}