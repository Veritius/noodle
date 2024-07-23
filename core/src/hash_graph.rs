//! [`HashMap`] backed [`Graph`] implementations.

use core::{hash::BuildHasher, cell::UnsafeCell};
use std::{collections::{HashMap, HashSet}, hash::RandomState, mem::transmute};
use crate::*;

/// A [`HashMap`] backed graph structure.
pub struct HashGraph<S: BuildHasher = RandomState> {
    nodes: HashMap<NodeId, Box<dyn Node>, S>,
    links: HashSet<LinkId, S>,

    node_idx: u32,
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

/// A [`HashMap`] backed graph structure.
/// 
/// Unlike [`HashGraph`], this uses [`UnsafeCell`] internally,
/// allowing it to implement the [`UnsafeGraph`] trait.
pub struct CellHashGraph<S: BuildHasher = RandomState> {
    nodes: HashMap<NodeId, Box<UnsafeCell<dyn Node>>, S>,
    links: HashSet<LinkId, S>,

    node_idx: u32,
}


impl<S: BuildHasher> Graph for CellHashGraph<S> {
    fn insert_node(&mut self, node: impl Into<Box<dyn Node>>) -> NodeId {
        let id = NodeId(self.node_idx);
        self.node_idx += 1;

        let node = unsafe {
            let node = node.into();
            // SAFETY: UnsafeCell<T> is #[repr(transparent)]
            transmute::<Box<dyn Node>, Box<UnsafeCell<dyn Node>>>(node)
        };

        self.nodes.insert(id, node);
        return id;
    }

    fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>> {
        let node = unsafe {
            let node = self.nodes.remove(&id)?;
            // SAFETY: UnsafeCell<T> is #[repr(transparent)]
            transmute::<Box<UnsafeCell<dyn Node>>, Box<dyn Node>>(node)
        };

        self.links.retain(|link| !(link.from.node == id) && !(link.to.node == id));
        return Some(node);
    }

    fn has_node(&self, id: NodeId) -> bool {
        return self.nodes.contains_key(&id);
    }

    fn get_node(&self, id: NodeId) -> Option<NodeRef> {
        self.nodes.get(&id).map(|v| NodeRef::from({
            // SAFETY: Guarantees are upheld by the user
            let v: &dyn Node = unsafe { &*(*v).get() };
            NodeRef::from(v)
        }))
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

impl<S: BuildHasher> UnsafeGraph for CellHashGraph<S> {
    type Access<'a> = CellHashGraphUnsafeAccess<'a, S> where S: 'a;

    unsafe fn get_unsafe(&mut self, id: NodeId) -> Self::Access<'_> {
        CellHashGraphUnsafeAccess {
            graph: self,
        }
    }
}

/// Permits unsafe access to a [`CellHashGraph`].
pub struct CellHashGraphUnsafeAccess<'a, S: BuildHasher> {
    graph: &'a mut CellHashGraph<S>,
}

impl<'a, S: BuildHasher> UnsafeGraphAccess<'a> for CellHashGraphUnsafeAccess<'a, S> {
    fn get_mut(&mut self, id: NodeId) -> Option<NodeMut> {
        self.graph.get_node_mut(id)
    }

    unsafe fn get_unsafe(&self, id: NodeId) -> Option<NodeMut> {
        todo!()
    }
}