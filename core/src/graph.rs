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
    fn insert_link(&mut self, link: NodeLinkId) -> Result<(), WouldCycle>;

    /// Inserts multiple links between a pair of nodes.
    /// May be faster than calling [`insert_link`](Graph::insert_link) repeatedly.
    fn insert_multiple_links(&mut self, from: NodeId, to: NodeId, iter: &mut dyn Iterator<Item = SocketLinkId>) -> Result<(), WouldCycle> {
        for link in iter {
            self.insert_link(NodeLinkId {
                left: NodeSocketId { node: from, socket: link.from },
                right: NodeSocketId { node: to, socket: link.to },
            })?;
        }

        return Ok(());
    }

    /// Removes a link between a pair of sockets on two nodes.
    fn remove_link(&mut self, link: NodeLinkId);

    /// Removes multiple links between a pair of nodes.
    /// May be faster than calling [`remove_link`](Graph::remove_link) repeatedly.
    fn remove_multiple_links(&mut self, from: NodeId, to: NodeId, iter: &mut dyn Iterator<Item = SocketLinkId>) {
        for link in iter {
            self.remove_link(NodeLinkId {
                left: NodeSocketId { node: from, socket: link.from },
                right: NodeSocketId { node: to, socket: link.to },
            });
        }
    }

    /// Returns `true` if a pair of sockets on two sides are linked.
    fn has_link(&self, link: NodeLinkId) -> bool;

    /// Returns the number of links the graph contains.
    /// Returns `None` if an estimate cannot be obtained.
    fn link_count(&self) -> Option<usize>;

    /// Solve for the output of `node` with the given [`OutputMask`].
    fn solve_node(
        &mut self,
        node: NodeId,
        outputs: OutputMask,
    ) -> Result<SocketValues, GraphSolveError>;
}

/// A traversal state that doesn't keep a borrow of the graph, permitting mutable access to the graph.
pub trait Walker<G, N>
where
    G: Graph<N> + ?Sized,
    N: Node,
{
    fn next(&mut self, graph: &G) -> Option<NodeId>;
}

pub trait WalkDependencies<N>
where
    Self: Graph<N>,
    N: Node,
{
    type Walker: Walker<Self, N>;

    fn walk_dependencies(&self, from: NodeId) -> Option<Self::Walker>;
}

pub trait WalkDependents<N>
where
    Self: Graph<N>,
    N: Node,
{
    type Walker: Walker<Self, N>;

    fn walk_dependents(&self, from: NodeId) -> Option<Self::Walker>;
}