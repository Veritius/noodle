//! A graph based on a hash map.

use core::fmt::Debug;
use std::marker::PhantomData;
use hashbrown::HashMap;
use noodle_core::*;
use crate::{visited::Visited, Link, Links, Vertex};

/// A simple directed acyclic graph structure based on a [`HashMap`].
pub struct HashGraph<N, NM = (), EM = ()> {
    last_idx: u32,
    vertices: HashMap<NodeId, Vertex<N, NM>>,
    links: HashMap<[NodeId; 2], Links<EM>>,
}

impl<N, NM, EM> Default for HashGraph<N, NM, EM> {
    fn default() -> Self {
        Self {
            last_idx: 0,
            vertices: HashMap::default(),
            links: HashMap::default(),
        }
    }
}

impl<N, NM, EM> HashGraph<N, NM, EM> {
    #[inline]
    fn next_node_id(&mut self) -> NodeId {
        let v = self.last_idx;
        self.last_idx += 1;
        return NodeId(v);
    }

    /// Inserts a vertex into the graph.
    pub fn insert_vertex(&mut self, vertex: Vertex<N, NM>) -> NodeId {
        let id = self.next_node_id();

        self.vertices.insert(id, vertex);

        return id;
    }

    /// Removes a vertex from the graph, severing any links.
    /// Returns an iterator over links that were severed.
    pub fn remove_vertex(&mut self, vertex: NodeId) -> Option<(Vertex<N, NM>, SeveredLinks<EM>)> {
        // We can early return if the vertex doesn't exist,
        // since that means there are no links to it
        let vtx = self.vertices.remove(&vertex)?;

        let extract = self.links.extract_if(|k, _| {
            // Simple comparison function
            k[0] == vertex || k[1] == vertex
        });

        // Converts the various ids to a collection of NodeLinkIds
        let iter = extract
            .flat_map(|([l, r], s)| {
                s.iter().map(|i| NodeLinkId {
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

        return Some((vtx, severed))
    }

    /// Immutably borrows a vertex from the graph.
    #[inline]
    pub fn get_vertex(&self, vertex: NodeId) -> Option<&Vertex<N, NM>> {
        self.vertices.get(&vertex)
    }

    /// Mutably borrows a vertex from the graph.
    #[inline]
    pub fn get_vertex_mut(&mut self, vertex: NodeId) -> Option<&mut Vertex<N, NM>> {
        self.vertices.get_mut(&vertex)
    }

    /// Returns the number of vertices in the graph.
    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns `true` if the node corresponding to `id` exists in the graph.
    #[inline]
    pub fn has_vertex(&self, id: NodeId) -> bool {
        self.vertices.contains_key(&id)
    }

    /// Create a link between two sockets on two nodes.
    /// Returns `Err` if creating this link would cause a cycle.
    pub fn insert_link_with_meta(&mut self, link: NodeLinkId, meta: Option<EM>) -> Result<(), WouldCycle> {
        todo!()
    }

    /// Removes a link from the graph.
    pub fn remove_link_with_meta(&mut self, link: NodeLinkId) -> Option<Links<EM>> {
        todo!()
    }

    /// Borrow [`Edges`] if it exists.
    #[inline]
    pub fn get_edges(&self, left: NodeId, right: NodeId) -> Option<&Links<EM>> {
        self.links.get(&[left, right])
    }

    /// Mutably borrow [`Edges`] if it exists.
    #[inline]
    fn get_edges_mut(&mut self, left: NodeId, right: NodeId) -> Option<&mut Links<EM>> {
        self.links.get_mut(&[left, right])
    }

    /// Mutably borrow, or try to create, an [`Edges`].
    /// If the edge does not exist, and creating it would create a cycle, this returns an error.
    pub fn get_or_insert_edges(&mut self, left: NodeId, right: NodeId) -> Result<&mut Links<EM>, WouldCycle> {
        // If it exists, return it as per usual.
        if let Some(edges) = self.get_edges_mut(left, right) { return Ok(edges); }

        todo!()
    }

    /// Iterates over the direct dependencies of `node`. Does not recurse.
    pub fn iter_direct_dependencies(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.links.iter()
            .filter(move |([_, r], _)| *r == node)
            .map(|([l, _], _)| *l)
    }

    /// Returns an iterator over all [`Edges`] items in the graph.
    #[inline]
    pub fn iter_edge_sets(&self) -> impl Iterator<Item = &Links<EM>> {
        self.links.values()
    }

    /// Counts the number of links across all edge sets.
    pub fn edge_count(&self) -> usize {
        self.iter_edge_sets().map(|v| v.count()).sum()
    }
}

impl<V: Debug, E: Debug> Debug for HashGraph<V, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HashGraph")
        .field("vertices", &self.vertices)
        .field("edges", &self.links)
        .finish()
    }
}

impl<N: Node, E> Graph for HashGraph<N, E> {
    type N = N;

    #[inline]
    fn insert_node(&mut self, node: impl Into<Self::N>) -> NodeId {
        self.insert_vertex(Vertex {
            node: node.into(),
            meta: None,
        })
    }

    #[inline]
    fn remove_node(&mut self, id: NodeId) -> Option<Self::N> {
        self.remove_vertex(id).map(|(v, _)| v.node)
    }

    #[inline]
    fn has_node(&self, id: NodeId) -> bool {
        self.has_vertex(id)
    }

    #[inline]
    fn get_node(&self, id: NodeId) -> Option<NodeRef<Self::N>> {
        self.get_vertex(id).map(|v| NodeRef::from(&v.node))
    }

    #[inline]
    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut<Self::N>> {
        self.get_vertex_mut(id).map(|v| NodeMut::from(&mut v.node))
    }

    #[inline]
    fn node_count(&self) -> Option<usize> {
        Some(self.vertex_count())
    }

    fn insert_link(&mut self, link: NodeLinkId) -> Result<(), WouldCycle> {
        self.insert_link_with_meta(link, None)
    }

    fn remove_link(&mut self, link: NodeLinkId) {
        self.remove_link_with_meta(link);
    }

    fn has_link(&self, link: NodeLinkId) -> bool {
        todo!()
    }

    #[inline]
    fn link_count(&self) -> Option<usize> {
        Some(self.edge_count())
    }

    fn solve_node(
        &mut self,
        node: NodeId,
        outputs: OutputMask,
    ) -> Result<SocketValues, GraphSolveError> {
        todo!()
    }
}

/// An iterator over links severed by [`remove_vertex`](HashGraph::remove_vertex) and related functions.
pub struct SeveredLinks<'a, EdgeMeta> {
    index: usize,
    items: Box<[NodeLinkId]>,

    // these may appear redundant, and they are,
    // but they let us dramatically change the
    // internals of this iterator without
    // needing to make breaking changes
    _p1: PhantomData<&'a ()>,
    _p2: PhantomData<EdgeMeta>,
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

impl<N, NM, EM> WalkDirectDependencies for HashGraph<N, NM, EM>
where
    HashGraph<N, NM, EM>: Graph,
{
    type Walker = HashGraphWalkDirectDependencies<N, NM, EM>;

    fn walk_direct_dependencies(&self, node: NodeId) -> Option<Self::Walker> {
        todo!()
    }
}

/// A walker over a node's direct dependencies, from a [`HashGraph`].
pub struct HashGraphWalkDirectDependencies<N, NM, EM> {
    _p1: PhantomData<(N, NM, EM)>,
}

impl<N, NM, EM> Walker for HashGraphWalkDirectDependencies<N, NM, EM> {
    type Context<'a> = () where Self: 'a;

    fn next<'a>(&'a mut self, context: Self::Context<'a>) -> Option<NodeId> {
        todo!()
    }
}

impl<N, NM, EM> WalkDirectDependents for HashGraph<N, NM, EM>
where
    HashGraph<N, NM, EM>: Graph,
{
    type Walker = HashGraphWalkDirectDependencies<N, NM, EM>;

    fn walk_direct_dependents(&self, node: NodeId) -> Option<Self::Walker> {
        todo!()
    }
}

/// A walker over nodes directly dependent on a node, from a [`HashGraph`].
pub struct HashGraphWalkDirectDependents<N, NM, EM> {
    _p1: PhantomData<(N, NM, EM)>,
}

impl<N, NM, EM> Walker for HashGraphWalkDirectDependents<N, NM, EM> {
    type Context<'a> = () where Self: 'a;

    fn next<'a>(&'a mut self, context: Self::Context<'a>) -> Option<NodeId> {
        todo!()
    }
}

/// A depth-first-search implementation for [`HashGraph`].
pub struct HashGraphDfs<N, NM, EM> {
    stack: Vec<NodeId>,
    discovered: Visited,

    _p1: PhantomData<(N, NM, EM)>,
}

impl<N, NM, EM> HashGraphDfs<N, NM, EM> {
    fn new(graph: &HashGraph<N, NM, EM>, start: NodeId) -> Self {
        let mut stack = Vec::with_capacity(1);
        stack.push(start);

        Self {
            stack,
            discovered: Visited::new(),

            _p1: PhantomData,
        }
    }
}

impl<N, NM, EM> Walker for HashGraphDfs<N, NM, EM> {
    type Context<'a> = &'a HashGraph<N, NM, EM> where N: 'a, NM: 'a, EM: 'a;

    fn next<'a>(&'a mut self, graph: Self::Context<'a>) -> Option<NodeId> {
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