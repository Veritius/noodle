use super::*;

/// A directed acyclic graph structure.
pub trait Graph<N: Node> {
    /// Add a node to the graph.
    fn insert_node(&mut self, node: impl Into<N>) -> NodeId;

    /// Remove a node from the graph, breaking any links.
    fn remove_node(&mut self, id: NodeId) -> Option<N>;

    /// Check if the node is present.
    fn has_node(&self, id: NodeId) -> bool;

    /// Immutably access a node by its ID.
    fn get_node(&self, id: NodeId) -> Option<NodeRef<N>>;

    /// Mutably access a node by its ID.
    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut<N>>;

    /// Returns the number of nodes the graph contains.
    /// Returns `None` if an estimate cannot be obtained.
    fn node_count(&self) -> Option<usize>;

    /// Inserts a link between a pair of sockets on two nodes.
    fn insert_link(&mut self, id: LinkId) -> Result<(), WouldCycle>;

    /// Removes a link between a pair of sockets on two nodes.
    fn remove_link(&mut self, id: LinkId);

    /// Returns `true` if a pair of sockets on two sides are linked.
    fn has_link(&self, id: LinkId) -> bool;

    /// Returns the number of links the graph contains.
    /// Returns `None` if an estimate cannot be obtained.
    fn link_count(&self) -> Option<usize>;

    /// Solve for the output of `node` with the given [`OutputMask`].
    fn solve_node(
        &mut self,
        node: NodeId,
        outputs: OutputMask,
    ) -> Result<SocketValues, ()>;
}