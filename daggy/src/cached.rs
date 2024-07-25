use noodle_core::*;
use crate::SimpleGraph;

/// A [`SimpleGraph`] that caches node outputs to minimise recalculations.
pub struct CachedGraph {
    graph: SimpleGraph,
}

impl CachedGraph {
    /// Creates a new [`CachedGraph`].
    pub fn new() -> Self {
        Self {
            graph: SimpleGraph::new(),
        }
    }
}

impl Graph for CachedGraph {
    #[inline]
    fn insert_node(&mut self, node: impl Into<Box<dyn Node>>) -> NodeId {
        self.graph.insert_node(node)
    }

    fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>> {
        self.graph.remove_node(id)
    }

    #[inline]
    fn has_node(&self, id: NodeId) -> bool {
        self.graph.has_node(id)
    }

    #[inline]
    fn get_node(&self, id: NodeId) -> Option<NodeRef> {
        self.graph.get_node(id)
    }

    #[inline]
    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut> {
        self.graph.get_node_mut(id)
    }

    #[inline]
    fn node_count(&self) -> Option<usize> {
        self.graph.node_count()
    }

    #[inline]
    fn reserve_nodes(&mut self, amt: usize) {
        self.graph.reserve_nodes(amt)
    }

    #[inline]
    fn reserve_nodes_exact(&mut self, amt: usize) {
        self.graph.reserve_nodes_exact(amt)
    }

    #[inline]
    fn insert_link(&mut self, id: LinkId) -> Result<(), WouldCycle> {
        self.graph.insert_link(id)
    }

    #[inline]
    fn remove_link(&mut self, id: LinkId) {
        self.graph.remove_link(id)
    }

    #[inline]
    fn has_link(&self, id: LinkId) -> bool {
        self.graph.has_link(id)
    }

    #[inline]
    fn link_count(&self) -> Option<usize> {
        self.graph.link_count()
    }

    #[inline]
    fn reserve_links(&mut self, amt: usize) {
        self.graph.reserve_links(amt)
    }

    #[inline]
    fn reserve_links_exact(&mut self, amt: usize) {
        self.graph.reserve_links_exact(amt)
    }

    fn solve_node(
        &mut self,
        node: NodeId,
        outputs: OutputMask,
    ) -> Result<SocketValues, ()> {
        todo!()
    }
}

impl CachingGraph for CachedGraph {
    fn clear_cached_output(&mut self, node: NodeId) {
        todo!()
    }
}