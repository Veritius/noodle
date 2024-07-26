use std::ops::{Deref, DerefMut};
use noodle_core::*;
use crate::linked::HashGraph;

/// A simple [`Graph`] implementation.
pub struct SimpleGraph<N> {
    graph: HashGraph<N>,
}

impl<N: Node> Graph<N> for SimpleGraph<N> {
    #[inline]
    fn insert_node(&mut self, node: impl Into<N>) -> NodeId {
        self.graph.insert_vertex(node.into())
    }

    #[inline]
    fn remove_node(&mut self, id: NodeId) -> Option<N> {
        self.graph.remove_vertex(id).map(|(n, _)| n)
    }

    #[inline]
    fn has_node(&self, id: NodeId) -> bool {
        self.graph.contains_vertex(id)
    }

    fn get_node(&self, id: NodeId) -> Option<NodeRef<N>> {
        self.graph.get_vertex(id).map(|v| NodeRef::from(v.deref()))
    }

    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut<N>> {
        self.graph.get_vertex_mut(id).map(|v| NodeMut::from(v.deref_mut()))
    }

    #[inline]
    fn node_count(&self) -> Option<usize> {
        Some(self.graph.vertex_count())
    }

    #[inline]
    fn insert_link(&mut self, link: NodeLinkId) -> Result<(), WouldCycle> {
        self.graph.insert_link(link)
    }

    #[inline]
    fn remove_link(&mut self, link: NodeLinkId) {
        self.graph.remove_link(link)
    }

    fn has_link(&self, link: NodeLinkId) -> bool {
        if let Some(edges) = self.graph.get_edges(link.left.node, link.right.node) {
            return edges.contains(SocketLinkId { from: link.left.socket, to: link.right.socket });
        } else {
            return false;
        }
    }

    #[inline]
    fn link_count(&self) -> Option<usize> {
        Some(self.graph.link_count())
    }

    fn solve_node(
        &mut self,
        node: NodeId,
        outputs: OutputMask,
    ) -> Result<SocketValues, ()> {
        todo!()
    }
}