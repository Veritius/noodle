//! Implementations for the petgraph crate.

use ::petgraph::{graph::{IndexType, NodeIndex}, stable_graph::StableGraph, Directed};
use crate::*;

unsafe impl IndexType for NodeId {
    fn new(x: usize) -> Self {
        Self(x)
    }

    fn index(&self) -> usize {
        self.0
    }

    fn max() -> Self {
        Self(usize::MAX)
    }
}

impl Graph for StableGraph<Box<dyn Node>, (), Directed, NodeId> {
    fn insert_node(&mut self, node: impl Into<Box<dyn Node>>) -> NodeId {
        NodeId(self.add_node(node.into()).index())
    }

    fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>> {
        self.remove_node(NodeIndex::new(id.index()))
    }

    fn has_node(&self, id: NodeId) -> bool {
        self.contains_node(NodeIndex::new(id.index()))
    }

    fn get_node(&self, id: NodeId) -> Option<NodeRef> {
        self.node_weight(NodeIndex::new(id.index()))
            .map(|v| NodeRef::from(&**v))
    }

    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut> {
        // self.node_weight_mut(NodeIndex::new(id.index()))
        //     .map(move |v| NodeMut::from(&mut **v))
        todo!()
    }

    unsafe fn get_node_unsafe(&self, id: NodeId) -> Option<NodeMut> {
        todo!()
    }

    fn reserve_nodes(&mut self, amt: usize) {
        todo!()
    }

    fn reserve_nodes_exact(&mut self, amt: usize) {
        todo!()
    }

    fn insert_link(&mut self, from: NodeSocketId, to: NodeSocketId) {
        todo!()
    }

    fn remove_link(&mut self, from: NodeSocketId, to: NodeSocketId) {
        todo!()
    }

    fn has_link(&self, from: NodeSocketId, to: NodeSocketId) -> bool {
        todo!()
    }

    fn get_link(&self, from: NodeSocketId, to: NodeSocketId) -> Option<Link> {
        todo!()
    }
}