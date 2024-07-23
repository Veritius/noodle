use crate::*;
use ::daggy;
use daggy::stable_dag::StableDag;

/// A [`Graph`] implementation based on `daggy`'s [`StableDag`] type.
pub struct VectorGraph {
    inner: StableDag<Box<dyn Node>, [SocketId; 2], NodeId>
}

impl Graph for VectorGraph {
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

    fn reserve_nodes(&mut self, amt: usize) {
        todo!()
    }

    fn reserve_nodes_exact(&mut self, amt: usize) {
        todo!()
    }

    fn insert_link(&mut self, id: LinkId) {
        todo!()
    }

    fn remove_link(&mut self, id: LinkId) {
        todo!()
    }

    fn has_link(&self, id: LinkId) -> bool {
        todo!()
    }

    fn reserve_links(&mut self, amt: usize) {
        todo!()
    }

    fn reserve_links_exact(&mut self, amt: usize) {
        todo!()
    }
}