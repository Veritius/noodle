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
    edges: HashMap<[NodeId; 2], Edges<Edge>>,
}

impl<Vertex, Edge> HashGraph<Vertex, Edge> {
    pub fn insert_vertex(&mut self, vertex: Vertex) -> NodeId {
        todo!()
    }

    pub fn remove_vertex(&mut self, vertex: NodeId) -> Option<Vertex> {
        todo!()
    }

    pub fn get_vertex(&self, vertex: NodeId) -> Option<&VertexItem<Vertex>> {
        todo!()
    }

    pub fn get_mut_vertex(&mut self, vertex: NodeId) -> Option<&mut VertexItem<Vertex>> {
        todo!()
    }

    pub fn insert_link(&mut self, from: NodeSocketId, to: NodeSocketId) -> Result<(), WouldCycle> {
        todo!()
    }

    pub fn remove_link(&mut self, from: NodeSocketId, to: NodeSocketId) {
        todo!()
    }
}

/// A vertex entry in a [`HashGraph`].
pub struct VertexItem<Vertex> {
    item: Vertex,
}

/// A set of edges between two [`VertexItem`] objects in a [`HashGraph`].
pub struct Edges<Edge> {
    edges: SmallVec<[EdgeItem<Edge>; 1]>,
}

impl<Edge> Edges<Edge> {
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