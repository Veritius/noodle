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
    fn insert_link(&mut self, id: LinkId);

    /// Removes a link between a pair of sockets on two nodes.
    fn remove_link(&mut self, id: LinkId);

    /// Returns `true` if a pair of sockets on two sides are linked.
    fn has_link(&self, id: LinkId) -> bool;

    /// Reserve space for at least `amt` nodes.
    /// Does nothing if capacity is already sufficient.
    fn reserve_links(&mut self, amt: usize);

    /// Reserve space for at least `amt` nodes.
    /// Does nothing if capacity is already sufficient.
    /// 
    /// Unlike [`reserve_nodes`](Self::reserve_nodes), this will not
    /// deliberately over-allocate to speculatively avoid frequent allocations.
    fn reserve_links_exact(&mut self, amt: usize);
}

/// Permits access to a [`Graph`] for advanced usage.
pub trait UnsafeGraph: Graph {
    /// The type returned to allow mutation.
    type Access<'a>: UnsafeGraphAccess<'a> where Self: 'a;

    /// Gets access to an API surface for unsafe [`Graph`] mutation.
    unsafe fn get_unsafe(&mut self, id: NodeId) -> Self::Access<'_>;
}

/// The access component of [`UnsafeGraph`].
/// 
/// This exists to hide a lot of the API of [`Graph`] to prevent accidental undefined behavior, such as:
/// - Safe code using [`Graph::get_node`] while a [`NodeMut`] from [`UnsafeGraphAccess::get_unsafe`] exists.
/// 
/// Implementing `Graph` on any type that implements this trait is a **really bad idea.**
/// This is because the behavior this trait exists to prevent becomes possible.
pub trait UnsafeGraphAccess<'a> {
    /// Get mutable access normally.
    /// 
    /// # Safety
    /// This is safe because the `&mut self` ensures that this is the only access.
    fn get_mut(&mut self, id: NodeId) -> Option<NodeMut>;

    /// Get mutable access through interior mutability.
    /// 
    /// This is intended for parallel tasks, such as implementations of [`ParSolver`].
    /// 
    /// # Safety
    /// While using this method, you are responsible for upholding Rust's mutability rules yourself.
    /// No matter what, the [aliasing rules](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html) must be followed.
    /// For example, conflicting accesses to the node behind `id` is **unsafe**, such as two mutable references, or
    /// one mutable and one immutable reference. This is only obtainable due to internal mutability used by the implementor.
    unsafe fn get_unsafe(&self, id: NodeId) -> Option<NodeMut>;
}