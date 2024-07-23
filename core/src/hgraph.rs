use core::hash::BuildHasher;
use std::{collections::{HashMap, HashSet}, hash::RandomState};
use crate::*;

/// A [`HashMap`] backed graph structure.
pub struct HashGraph<S: BuildHasher = RandomState> {
    nodes: HashMap<NodeId, Box<dyn Node>, S>,
    links: HashSet<LinkId, S>,

    node_idx: usize,
}

impl<S: BuildHasher> Graph for HashGraph<S> {
    fn insert_node(&mut self, node: impl Into<Box<dyn Node>>) -> NodeId {
        let id = NodeId(self.node_idx);
        self.node_idx += 1;

        self.nodes.insert(id, node.into());
        return id;
    }

    fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>> {
        let node = self.nodes.remove(&id)?;
        self.links.retain(|link| !(link.from.node == id) && !(link.to.node == id));
        return Some(node);
    }

    #[inline]
    fn has_node(&self, id: NodeId) -> bool {
        return self.nodes.contains_key(&id);
    }

    fn get_node(&self, id: NodeId) -> Option<NodeRef> {
        self.nodes.get(&id).map(|v| NodeRef::from(&**v))
    }

    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut> {
        todo!()
    }

    fn reserve_nodes(&mut self, amt: usize) {
        self.nodes.reserve(amt);
    }

    #[inline]
    fn reserve_nodes_exact(&mut self, amt: usize) {
        self.reserve_nodes(amt);
    }

    fn insert_link(&mut self, id: LinkId) {
        self.links.insert(id);
    }

    fn remove_link(&mut self, id: LinkId) {
        self.links.remove(&id);
    }

    fn has_link(&self, id: LinkId) -> bool {
        return self.links.contains(&id);
    }

    fn reserve_links(&mut self, amt: usize) {
        self.links.reserve(amt);
    }

    #[inline]
    fn reserve_links_exact(&mut self, amt: usize) {
        self.reserve_links(amt);
    }
}