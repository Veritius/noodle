use crate::*;

/// Returns a [`Walker`] over the dependencies of a given node.
pub trait WalkDependencies: Graph {
    /// The returned walker.
    type Walker: Walker;

    /// Returns an object that can be used to iterate over the dependencies of a `node`.
    fn walk_dependencies(&self, node: NodeId) -> Option<Self::Walker>;
}

/// Returns a [`Walker`] over the nodes dependent on a given node.
pub trait WalkDependents: Graph {
    /// The returned walker.
    type Walker: Walker;

    /// Returns an object that can be used to iterate over the nodes dependent on `node`.
    fn walk_dependents(&self, node: NodeId) -> Option<Self::Walker>;
}

/// A traversal state that doesn't keep a borrow of the graph, permitting mutable access to the graph.
pub trait Walker {
    /// Context given to [`next`](Walker::next) to progress.
    type Context;

    /// Walks to the next node in the graph.
    fn next(&mut self, graph: Self::Context) -> Option<NodeId>;
}