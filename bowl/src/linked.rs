use core::fmt::Debug;
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
pub struct HashGraph<V, E = ()> {
    last_idx: u32,
    vertices: HashMap<NodeId, Vertex<V>>,
    edges: HashMap<[NodeId; 2], Edges<E>>,
}

impl<V, E> HashGraph<V, E> {
    #[inline]
    fn next_node_id(&mut self) -> NodeId {
        let v = self.last_idx;
        self.last_idx += 1;
        return NodeId(v);
    }

    /// Inserts a vertex into the graph.
    pub fn insert_vertex(&mut self, vertex: V) -> NodeId {
        let id = self.next_node_id();

        self.vertices.insert(id, Vertex {
            item: vertex
        });

        return id;
    }

    /// Removes a vertex from the graph, severing any links.
    /// Returns an iterator over links that were severed.
    pub fn remove_vertex(&mut self, vertex: NodeId) -> Option<(V, SeveredLinks<E>)> {
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
    pub fn get_vertex(&self, vertex: NodeId) -> Option<&Vertex<V>> {
        self.vertices.get(&vertex)
    }

    /// Mutably borrows a vertex from the graph.
    #[inline]
    pub fn get_vertex_mut(&mut self, vertex: NodeId) -> Option<&mut Vertex<V>> {
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

    /// Borrow [`Edges`] if it exists.
    #[inline]
    pub fn get_edges(&self, left: NodeId, right: NodeId) -> Option<&Edges<E>> {
        self.edges.get(&[left, right])
    }

    /// Mutably borrow [`Edges`] if it exists.
    #[inline]
    pub fn get_edges_mut(&mut self, left: NodeId, right: NodeId) -> Option<&mut Edges<E>> {
        self.edges.get_mut(&[left, right])
    }

    /// Mutably borrow, or try to create, an [`Edges`].
    /// If the edge does not exist, and creating it would create a cycle, this returns an error.
    pub fn get_or_insert_edges(&mut self, left: NodeId, right: NodeId) -> Result<&mut Edges<E>, WouldCycle> {
        todo!()
    }

    /// Recursively iterates over the dependencies of `node`.
    /// If you don't want to recurse, use [`iter_direct_dependencies`](Self::iter_direct_dependencies).
    pub fn iter_dependencies(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        return [].iter().cloned() // TODO
    }

    /// Iterates over the direct dependencies of `node`. Does not recurse.
    pub fn iter_direct_dependencies(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.edges.iter()
            .filter(move |([_, r], _)| *r == node)
            .map(|([l, _], _)| *l)
    }

    /// Recursively iterates over the nodes dependent on `node`.
    /// If you don't want to recurse, use [`iter_direct_dependents`](Self::iter_direct_dependents).
    pub fn iter_dependents(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        return [].iter().cloned() // TODO
    }

    /// Iterates over the nodes directly dependent on `node`. Does not recurse.
    pub fn iter_direct_dependents(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.edges.iter()
            .filter(move |([l, _], _)| *l == node)
            .map(|([_, r], _)| *r)
    }

    /// Returns an iterator over all [`Edges`] items in the graph.
    #[inline]
    pub fn iter_edge_sets(&self) -> impl Iterator<Item = &Edges<E>> {
        self.edges.values()
    }

    /// Counts the number of links across all edge sets.
    pub fn link_count(&self) -> usize {
        self.iter_edge_sets().map(|v| v.count()).sum()
    }
}

impl<V: Debug, E: Debug> Debug for HashGraph<V, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HashGraph")
        .field("vertices", &self.vertices)
        .field("edges", &self.edges)
        .finish()
    }
}

/// A vertex entry in a [`HashGraph`].
pub struct Vertex<V> {
    item: V,
}

impl<V> Deref for Vertex<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<V> DerefMut for Vertex<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl<V: Debug> Debug for Vertex<V> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.item.fmt(f)
    }
}

/// A set of edges between two [`Vertex`] objects in a [`HashGraph`].
pub struct Edges<E> {
    edges: SmallVec<[EdgeItem<E>; 1]>,
}

impl<E> Edges<E> {
    /// Manually adds an edge between two nodes.
    /// 
    /// # SAFETY
    /// Adding an edge **must not** create a cycle in the graph.
    /// The connection requirements of the socket shape must be followed.
    pub unsafe fn insert(&mut self, id: SocketLinkId, edge: E) {
        if let Err(idx) = self.edges.binary_search_by(|v| v.id.cmp(&id)) {
            self.edges.insert(idx, EdgeItem { id, edge });
        }
    }

    /// Manually removes an edge between two nodes.
    pub fn remove(&mut self, id: SocketLinkId) -> Option<E> {
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

impl<E: Debug> Debug for Edges<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.edges.fmt(f)
    }
}

struct EdgeItem<Edge> {
    id: SocketLinkId,
    edge: Edge,
}

impl<E: Debug> Debug for EdgeItem<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EdgeItem")
        .field("id", &self.id)
        .field("edge", &self.edge)
        .finish()
    }
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

struct Visited(Vec<NodeId>);

impl Visited {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn with_capacity(amt: usize) -> Self {
        Self(Vec::with_capacity(amt))
    }

    fn visit(&mut self, id: NodeId) -> bool {
        match self.0.binary_search(&id) {
            Ok(_) => { return false },

            Err(idx) => {
                self.0.insert(idx, id);
                return true;
            },
        }
    }

    #[inline]
    fn is_visited(&self, id: NodeId) -> bool {
        self.0.binary_search(&id).is_ok()
    }
}

struct Dfs {
    stack: Vec<NodeId>,
    discovered: Visited,
}

impl Dfs {
    fn new<V, E>(graph: &HashGraph<V, E>) -> Self {
        Self {
            stack: Vec::new(),
            discovered: Visited::new(),
        }
    }

    fn next<V, E>(&mut self, graph: &HashGraph<V, E>) -> Option<NodeId> {
        while let Some(node) = self.stack.pop() {
            if self.discovered.visit(node) {
                for next in graph.iter_direct_dependencies(node) {
                    if !self.discovered.is_visited(next) {
                        self.stack.push(next);
                    }
                }

                return Some(node);
            }
        }

        return None;
    }
}

struct DfsPostOrder {
    stack: Vec<NodeId>,
    discovered: Visited,
    finished: Visited,
}

impl DfsPostOrder {
    fn new<V, E>(graph: &HashGraph<V, E>) -> Self {
        Self {
            stack: Vec::new(),
            discovered: Visited::new(),
            finished: Visited::new(),
        }
    }

    fn next<V, E>(&mut self, graph: &HashGraph<V, E>) -> Option<NodeId> {
        while let Some(next) = self.stack.last().cloned() {
            if self.discovered.visit(next) {
                for sp in graph.iter_direct_dependencies(next) {
                    if !self.discovered.is_visited(sp) {
                        self.stack.push(sp);
                    }
                }
            } else {
                self.stack.pop();
                if self.finished.visit(next) {
                    return Some(next);
                }
            }
        }

        return None;
    }
}