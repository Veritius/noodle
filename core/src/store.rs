use alloc::boxed::Box;
use super::*;

/// Data storage for a node graph.
pub trait Graph {
    /// Add a node to the graph.
    fn insert_node(&mut self, node: impl Into<Box<dyn Node>>) -> NodeId;

    /// Remove a node from the graph, breaking any links.
    fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>>;

    /// Check if the node is present.
    fn has_node(&self, id: NodeId) -> bool;

    /// Immutably access a node by its ID.
    fn get_node(&self, id: NodeId) -> Option<NodeRef>;

    /// Mutably access a node by its ID.
    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut>;

    /// Reserve space for at least `amt` nodes.
    /// Does nothing if capacity is already sufficient.
    fn reserve_nodes(&mut self, amt: usize);

    /// Reserve space for at least `amt` nodes.
    /// Does nothing if capacity is already sufficient.
    /// 
    /// Unlike [`reserve_nodes`](Self::reserve_nodes), this will not
    /// deliberately over-allocate to speculatively avoid frequent allocations.
    fn reserve_nodes_exact(&mut self, amt: usize);

    /// Inserts a link between a pair of sockets on two nodes.
    fn insert_link(&mut self, from: NodeSocketId, to: NodeSocketId);

    /// Removes a link between a pair of sockets on two nodes.
    fn remove_link(&mut self, from: NodeSocketId, to: NodeSocketId);

    /// Returns `true` if a pair of sockets on two sides are linked.
    fn has_link(&self, from: NodeSocketId, to: NodeSocketId) -> bool;

    /// Returns a reference to a link, if it exists.
    fn get_link(&self, from: NodeSocketId, to: NodeSocketId) -> Option<Link>;
}