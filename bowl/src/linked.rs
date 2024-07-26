use hashbrown::HashMap;
use noodle_core::*;

/// A simple directed acyclic graph structure based on a [`HashMap`].
/// 
/// This type stores multiple edges between vertices, identified with the `EdgeId` generic.
/// 
/// This is an internal type and does not implement the [`Graph`](noodle_core::Graph) trait.
/// You may get better use out of higher level types that do implement [`Graph`].
/// This is still exposed for the use of advanced users.
#[derive(Default)]
pub struct HashGraph<Vertex, Edge = (), VertexId = NodeId, EdgeId = SocketLinkId> {
    vertices: HashMap<VertexId, VertexItem<Vertex, Edge, EdgeId>>,
}

impl<Vertex, Edge, VertexId, EdgeId> HashGraph<Vertex, Edge, VertexId, EdgeId> {
    pub fn insert_vertex(&mut self, vertex: Vertex) -> VertexId {
        todo!()
    }

    pub fn remove_vertex(&mut self, vertex: VertexId) -> Option<Vertex> {
        todo!()
    }

    pub fn get_vertex(&self, vertex: VertexId) -> Option<&VertexItem<Vertex, Edge, EdgeId>> {
        todo!()
    }

    pub fn get_mut_vertex(&mut self, vertex: VertexId) -> Option<&mut VertexItem<Vertex, Edge, EdgeId>> {
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
pub struct VertexItem<Vertex, Edge, EdgeId> {
    item: Vertex,
    edges: HashMap<EdgeId, EdgeItem<Edge>>,
}

/// An edge entry in a [`HashGraph`].
pub struct EdgeItem<Edge> {
    item: Edge,
}