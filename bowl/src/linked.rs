use std::{marker::PhantomData, ops::{Deref, DerefMut}};
use hashbrown::HashMap;
use noodle_core::*;
use smallvec::SmallVec;

/// A simple directed acyclic graph structure based on a [`HashMap`].
/// 
/// This is an internal type and does not implement the [`Graph`](noodle_core::Graph) trait.
/// You may get better use out of higher level types that do implement [`Graph`].
/// This is still exposed for the use of advanced users.
#[derive(Default)]
pub struct HashGraph<Vertex, Edge = ()> {
    last_idx: u32,
    vertices: HashMap<NodeId, VertexItem<Vertex>>,
    edges: HashMap<[NodeId; 2], EdgeSet<Edge>>,
}

impl<Vertex, Edge> HashGraph<Vertex, Edge> {
    #[inline]
    fn next_node_id(&mut self) -> NodeId {
        let v = self.last_idx;
        self.last_idx += 1;
        return NodeId(v);
    }

    /// Inserts a vertex into the graph.
    pub fn insert_vertex(&mut self, vertex: Vertex) -> NodeId {
        let id = self.next_node_id();

        self.vertices.insert(id, VertexItem {
            item: vertex
        });

        return id;
    }

    /// Removes a vertex from the graph, severing any links.
    /// Returns an iterator over links that were severed.
    pub fn remove_vertex(&mut self, vertex: NodeId) -> Option<(Vertex, SeveredLinks<Edge>)> {
        // We can early return if the vertex doesn't exist,
        // since that means there are no links to it
        let vtx = self.vertices.remove(&vertex)?;

        let extract = self.edges.extract_if(|k, _| {
            // Simple comparison function
            k[0] == vertex || k[1] == vertex
        });

        // Converts the various ids to a collection of NodeLinkIds
        let iter = extract
            .flat_map(|([l, r], s)| {
                s.edges.iter().map(|i| NodeLinkId {
                    left: NodeSocketId { node: l, socket: i.id.from },
                    right: NodeSocketId { node: r, socket: i.id.to },
                }).collect::<Vec<_>>() // TODO: don't allocate
            })
            .collect::<Box<[_]>>();

        // Return the iterator over severed links
        let severed = SeveredLinks {
            index: 0,
            items: iter,
            
            _p1: PhantomData,
            _p2: PhantomData,
        };

        return Some((vtx.item, severed))
    }

    /// Immutably borrows a vertex from the graph.
    #[inline]
    pub fn get_vertex(&self, vertex: NodeId) -> Option<&VertexItem<Vertex>> {
        self.vertices.get(&vertex)
    }

    /// Mutably borrows a vertex from the graph.
    #[inline]
    pub fn get_vertex_mut(&mut self, vertex: NodeId) -> Option<&mut VertexItem<Vertex>> {
        self.vertices.get_mut(&vertex)
    }

    /// Returns the number of vertices in the graph.
    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns `true` if the node corresponding to `id` exists in the graph.
    #[inline]
    pub fn contains_vertex(&self, id: NodeId) -> bool {
        self.vertices.contains_key(&id)
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
    pub fn get_edges_mut(&mut self, left: NodeId, right: NodeId) -> Option<&mut EdgeSet<Edge>> {
        self.edges.get_mut(&[left, right])
    }

    /// Mutably borrow, or try to create, an [`EdgeSet`].
    /// If the edge does not exist, and creating it would create a cycle, this returns an error.
    pub fn get_or_insert_edges(&mut self, left: NodeId, right: NodeId) -> Result<&mut EdgeSet<Edge>, WouldCycle> {
        todo!()
    }

    /// Recursively iterates over the dependencies of `node`.
    /// If you don't want to recurse, use [`iter_direct_dependencies`](Self::iter_direct_dependencies).
    pub fn iter_dependencies(&self, node: NodeId) -> impl Iterator<Item = &Vertex> {
        return [].iter() // TODO
    }

    /// Iterates over the direct dependencies of `node`. Does not recurse.
    pub fn iter_direct_dependencies(&self, node: NodeId) -> impl Iterator<Item = &Vertex> {
        return [].iter() // TODO
    }

    /// Recursively iterates over the nodes dependent on `node`.
    /// If you don't want to recurse, use [`iter_direct_dependents`](Self::iter_direct_dependents).
    pub fn iter_dependents(&self, node: NodeId) -> impl Iterator<Item = &Vertex> {
        return [].iter() // TODO
    }

    /// Iterates over the nodes directly dependent on `node`. Does not recurse.
    pub fn iter_direct_dependents(&self, node: NodeId) -> impl Iterator<Item = &Vertex> {
        return [].iter() // TODO
    }

    /// Returns an iterator over all [`EdgeSet`] items in the graph.
    #[inline]
    pub fn iter_edge_sets(&self) -> impl Iterator<Item = &EdgeSet<Edge>> {
        self.edges.values()
    }

    /// Counts the number of links across all edge sets.
    pub fn link_count(&self) -> usize {
        self.iter_edge_sets().map(|v| v.count()).sum()
    }
}

/// A vertex entry in a [`HashGraph`].
pub struct VertexItem<Vertex> {
    item: Vertex,
}

impl<V> Deref for VertexItem<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<V> DerefMut for VertexItem<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
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

    /// Returns true if the link corresponding to `id` exists.
    pub fn contains(&self, id: SocketLinkId) -> bool {
        self.edges.binary_search_by(|v| v.id.cmp(&id)).is_ok()
    }

    /// Returns the number of links between the two nodes.
    #[inline]
    pub fn count(&self) -> usize {
        self.edges.len()
    }
}

struct EdgeItem<Edge> {
    id: SocketLinkId,
    edge: Edge,
}

/// An iterator over links severed by [`remove_vertex`](HashGraph::remove_vertex) and related functions.
pub struct SeveredLinks<'a, Edge> {
    index: usize,
    items: Box<[NodeLinkId]>,

    // these may appear redundant, and they are,
    // but they let us dramatically change the
    // internals of this iterator without
    // needing to make breaking changes
    _p1: PhantomData<&'a ()>,
    _p2: PhantomData<Edge>,
}

impl<E> Iterator for SeveredLinks<'_, E> {
    type Item = NodeLinkId;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if we've reached the end of the items
        if self.index == self.items.len() { return None }

        let item = self.items[self.index];
        self.index += 1; // keep track of where we're at

        return Some(item);
    }
}