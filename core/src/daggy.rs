use crate::*;
use ::daggy;
use daggy::stable_dag::StableDag;

type GraphInner = StableDag<Box<dyn Node>, [SocketId; 2], NodeId>;

/// A [`Graph`] implementation based on `daggy`'s [`StableDag`] type.
pub struct VectorGraph {
    inner: GraphInner,
}

impl AsRef<GraphInner> for VectorGraph {
    fn as_ref(&self) -> &GraphInner {
        &self.inner
    }
}

impl Graph for VectorGraph {
    fn insert_node(&mut self, node: impl Into<Box<dyn Node>>) -> NodeId {
        NodeId(self.inner.add_node(node.into()).index() as u32)
    }

    #[inline]
    fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>> {
        self.inner.remove_node(id.into())
    }

    #[inline]
    fn has_node(&self, id: NodeId) -> bool {
        self.inner.contains_node(id.into())
    }

    fn get_node(&self, id: NodeId) -> Option<NodeRef> {
        self.inner.node_weight(id.into())
            .map(|v| NodeRef::from(&**v))
    }

    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut> {
        self.inner.node_weight_mut(id.into())
            .map(move |v| NodeMut::from(v))
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

    // These don't do anything
    fn reserve_nodes(&mut self, _amt: usize) {}
    fn reserve_nodes_exact(&mut self, _amt: usize) {}
    fn reserve_links(&mut self, _amt: usize) {}
    fn reserve_links_exact(&mut self, _amt: usize) {}
}

impl<E> From<daggy::WouldCycle<E>> for WouldCycle {
    #[inline]
    fn from(_value: daggy::WouldCycle<E>) -> Self {
        WouldCycle
    }
}