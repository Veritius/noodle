use noodle_core::*;
use crate::UncachedGraph;

pub struct CachedGraph {
    queue: UncachedGraph,
}

impl CachedGraph {
    pub fn new() -> Self {
        Self {
            queue: UncachedGraph::new(),
        }
    }
}

impl Graph for CachedGraph {
    fn insert_node(&mut self, node: impl Into<Box<dyn Node>>) -> NodeId {
        todo!()
    }

    fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>> {
        todo!()
    }

    fn has_node(&self, id: NodeId) -> bool {
        todo!()
    }

    fn get_node(&self, id: NodeId) -> Option<NodeRef> {
        todo!()
    }

    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut> {
        todo!()
    }

    fn node_count(&self) -> Option<usize> {
        todo!()
    }

    fn reserve_nodes(&mut self, amt: usize) {
        todo!()
    }

    fn reserve_nodes_exact(&mut self, amt: usize) {
        todo!()
    }

    fn insert_link(&mut self, id: LinkId) -> Result<(), WouldCycle> {
        todo!()
    }

    fn remove_link(&mut self, id: LinkId) {
        todo!()
    }

    fn has_link(&self, id: LinkId) -> bool {
        todo!()
    }

    fn link_count(&self) -> Option<usize> {
        todo!()
    }

    fn reserve_links(&mut self, amt: usize) {
        todo!()
    }

    fn reserve_links_exact(&mut self, amt: usize) {
        todo!()
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