use daggy::stable_dag::StableDag;
use noodle_core::*;
use smallvec::SmallVec;
use crate::{edges::VectorGraphEdges, id::{node_id_to_node_index, node_index_to_node_id, NodeIdWrap}, SimpleGraph};

type GraphInner<N: Node> = StableDag<Box<CachedGraphNode<N>>, VectorGraphEdges, NodeIdWrap>;

/// A [`SimpleGraph`] that caches node outputs to minimise recalculations.
pub struct CachedGraph<N: Node> {
    inner: GraphInner<N>,
}

impl<N: Node> CachedGraph<N> {
    /// Creates a new [`CachedGraph`].
    pub fn new() -> Self {
        Self {
            inner: GraphInner::new(),
        }
    }
}

impl<N: Node> Graph<N> for CachedGraph<N> {
    fn insert_node(&mut self, node: impl Into<N>) -> NodeId {
        let node = Box::new(CachedGraphNode {
            node: node.into(),
            cached: SmallVec::new(),
        });

        node_index_to_node_id(self.inner.add_node(node))
    }

    fn remove_node(&mut self, id: NodeId) -> Option<N> {
        todo!()

        // self.inner.remove_node(node_id_to_node_index(id))
        //     .map(|node| node.ptr)
    }

    
    fn has_node(&self, id: NodeId) -> bool {
        todo!()
    }

    
    fn get_node(&self, id: NodeId) -> Option<NodeRef<N>> {
        todo!()
    }

    
    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut<N>> {
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

impl<N: Node> CachingGraph<N> for CachedGraph<N> {
    fn clear_cached_output(&mut self, node: NodeId) {
        todo!()
    }
}

struct CachedGraphNode<N> {
    node: N,
    cached: SmallVec<[SocketValue; 1]>,
}