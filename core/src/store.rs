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

    /// Immutably access a node by its ID, without checking that it exists.
    /// 
    /// # Safety
    /// - The node associated with `id` must exist.
    unsafe fn get_node_unchecked(&mut self, id: NodeId) -> NodeRef {
        self.get_node(id).unwrap_unchecked()
    }

    /// Mutably access a node by its ID.
    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut>;

    /// Mutably access a node by its ID, without checking that it exists.
    /// 
    /// # Safety
    /// - The node associated with `id` must exist.
    unsafe fn get_node_mut_unchecked(&mut self, id: NodeId) -> NodeMut {
        self.get_node_mut(id).unwrap_unchecked()
    }

    /// Mutably access a node by its ID with an immutable reference.
    /// 
    /// This is intended for use in parallel/concurrent operations.
    /// You are responsible for ensuring that mutable accesses do not overlap.
    /// 
    /// # Safety
    /// - This function must not be used to violate Rust's aliasing guarantees.
    /// - Structural changes (adding/removing nodes or links) must not occur while the reference is held.
    unsafe fn get_node_unsafe(&self, id: NodeId) -> Option<NodeMut>;

    /// Mutably access a node by its ID, without checking that it exists.
    /// 
    /// # Safety
    /// - The node associated with `id` must exist.
    /// - This function must not be used to violate Rust's aliasing guarantees.
    /// - Structural changes (adding/removing nodes or links) must not occur while the reference is held.
    unsafe fn get_node_unsafe_unchecked(&self, id: NodeId) -> NodeMut {
        self.get_node_unsafe(id).unwrap_unchecked()
    }

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
    fn insert_link(&mut self, from: LinkId, to: LinkId);

    /// Removes a link between a pair of sockets on two nodes.
    fn remove_link(&mut self, from: LinkId, to: LinkId);

    /// Returns `true` if a pair of sockets on two sides are linked.
    fn has_link(&self, from: LinkId, to: LinkId) -> bool;

    /// Returns a reference to a link, if it exists.
    fn get_link(&self, from: LinkId, to: LinkId) -> Option<Link>;

    /// Returns a reference to a link, without checking if it exists.
    /// 
    /// # Safety
    /// - The link between `from` and `to` must exist.
    unsafe fn get_link_unchecked(&self, from: LinkId, to: LinkId) -> Link {
        self.get_link(from, to).unwrap_unchecked()
    }
}