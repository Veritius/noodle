use hashbrown::HashMap;
use noodle_core::*;
use smallvec::SmallVec;

/// A simple directed acyclic graph structure based on a [`HashMap`].
/// 
/// This type stores multiple edges between vertices, identified with the `EdgeId` generic.
/// 
/// This is an internal type and does not implement the [`Graph`](noodle_core::Graph) trait.
/// You may get better use out of higher level types that do implement [`Graph`].
/// This is still exposed for the use of advanced users.
#[derive(Default)]
pub struct HashGraph<Vertex, Edge = ()> {
    vertices: HashMap<NodeId, VertexItem<Vertex>>,
    edges: HashMap<[NodeId; 2], EdgeSet<Edge>>,
}

impl<Vertex, Edge> HashGraph<Vertex, Edge> {
    /// Inserts a vertex into the graph.
    pub fn insert_vertex(&mut self, vertex: Vertex) -> NodeId {
        todo!()
    }

    /// Removes a vertex from the graph, severing any links.
    /// Returns an iterator over links that were severed.
    pub fn remove_vertex(&mut self, vertex: NodeId) -> Option<Vertex> {
        todo!()
    }

    /// Immutably borrows a vertex from the graph.
    pub fn get_vertex(&self, vertex: NodeId) -> Option<&VertexItem<Vertex>> {
        todo!()
    }

    /// Mutably borrows a vertex from the graph.
    pub fn get_mut_vertex(&mut self, vertex: NodeId) -> Option<&mut VertexItem<Vertex>> {
        todo!()
    }

    /// Create a link between two sockets on two nodes.
    /// Returns `Err` if creating this link would cause a cycle.
    pub fn insert_link(&mut self, link: NodeLinkId) -> Result<(), WouldCycle> {
        todo!()
    }

    /// Removes a link from the graph.
    pub fn remove_link(&mut self, link: NodeLinkId) {
        todo!()
    }

    /// Borrow an [`EdgeSet`] if it exists.
    #[inline]
    pub fn get_edges(&self, left: NodeId, right: NodeId) -> Option<&EdgeSet<Edge>> {
        self.edges.get(&[left, right])
    }

    /// Mutably borrow an [`EdgeSet`] if it exists.
    #[inline]
    pub fn get_mut_edges(&mut self, left: NodeId, right: NodeId) -> Option<&mut EdgeSet<Edge>> {
        self.edges.get_mut(&[left, right])
    }

    /// Mutably borrow, or try to create, an [`EdgeSet`].
    /// If the edge does not exist, and creating it would create a cycle, this returns an error.
    pub fn get_or_insert_edges(&mut self, left: NodeId, right: NodeId) -> Result<&mut EdgeSet<Edge>, WouldCycle> {
        todo!()
    }
}

/// A vertex entry in a [`HashGraph`].
pub struct VertexItem<Vertex> {
    item: Vertex,
}

/// A set of edges between two [`VertexItem`] objects in a [`HashGraph`].
pub struct EdgeSet<Edge> {
    edges: SmallVec<[EdgeItem<Edge>; 1]>,
}

impl<Edge> EdgeSet<Edge> {
    /// Manually adds an edge between two nodes.
    /// 
    /// # SAFETY
    /// Adding an edge **must not** create a cycle in the graph.
    /// The connection requirements of the socket shape must be followed.
    pub unsafe fn insert(&mut self, id: SocketLinkId, edge: Edge) {
        if let Err(idx) = self.edges.binary_search_by(|v| v.id.cmp(&id)) {
            self.edges.insert(idx, EdgeItem { id, edge });
        }
    }

    /// Manually removes an edge between two nodes.
    pub fn remove(&mut self, id: SocketLinkId) -> Option<Edge> {
        let idx = self.edges.binary_search_by(|v| v.id.cmp(&id)).ok()?;
        return Some(self.edges.remove(idx).edge);
    }
}

struct EdgeItem<Edge> {
    id: SocketLinkId,
    edge: Edge,
}