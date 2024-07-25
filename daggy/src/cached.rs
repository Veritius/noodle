use daggy::stable_dag::StableDag;
use noodle_core::*;
use smallvec::SmallVec;
use crate::{edges::VectorGraphEdges, id::{node_id_to_node_index, node_index_to_node_id, NodeIdWrap}, SimpleGraph};

type GraphInner = StableDag<Box<CachedGraphNode>, VectorGraphEdges, NodeIdWrap>;

/// A [`SimpleGraph`] that caches node outputs to minimise recalculations.
pub struct CachedGraph {
    inner: GraphInner,
}

impl CachedGraph {
    /// Creates a new [`CachedGraph`].
    pub fn new() -> Self {
        Self {
            inner: GraphInner::new(),
        }
    }
}

impl Graph for CachedGraph {
    
    fn insert_node(&mut self, node: impl Into<Box<dyn Node>>) -> NodeId {
        let node = Box::new(CachedGraphNode {
            ptr: node.into(),
            cached: SmallVec::new(),
        });

        node_index_to_node_id(self.inner.add_node(node))
    }

    fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>> {
        todo!()

        // self.inner.remove_node(node_id_to_node_index(id))
        //     .map(|node| node.ptr)
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

struct CachedGraphNode {
    ptr: Box<dyn Node>,
    cached: SmallVec<[SocketValue; 1]>,
}