use crate::*;

/// A traversal state that doesn't keep a borrow of the graph, permitting mutable access to the graph.
pub trait Walker<G, N>
where
    G: Graph<N> + ?Sized,
    N: Node,
{
    /// Walks to the next node in the graph.
    fn next(&mut self, graph: &G) -> Option<NodeId>;
}

/// Returns a [`Walker`] over the dependencies of a given node.
pub trait WalkDependencies<N>
where
    Self: Graph<N>,
    N: Node,
{
    /// The returned walker.
    type Walker: Walker<Self, N>;

    /// Returns an object that can be used to iterate over the dependencies of a `node`.
    fn walk_dependencies(&self, node: NodeId) -> Option<Self::Walker>;
}

/// Returns a [`Walker`] over the nodes dependent on a given node.
pub trait WalkDependents<N>
where
    Self: Graph<N>,
    N: Node,
{
    /// The returned walker.
    type Walker: Walker<Self, N>;

    /// Returns an object that can be used to iterate over the nodes dependent on `node`.
    fn walk_dependents(&self, node: NodeId) -> Option<Self::Walker>;
}