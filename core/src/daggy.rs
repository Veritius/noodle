use crate::*;
use ::daggy;
use daggy::{stable_dag::StableDag, EdgeIndex};
use smallvec::SmallVec;

type GraphInner = StableDag<Box<dyn Node>, VectorGraphEdges, NodeId>;

/// A [`Graph`] implementation based on `daggy`'s [`StableDag`] type.
pub struct VectorGraph {
    inner: GraphInner,
}

// public interface
impl VectorGraph {
    /// Creates a new, empty [`VectorGraph`].
    pub fn new() -> Self {
        Self {
            inner: GraphInner::new(),
        }
    }
}

// internal stuff
impl VectorGraph {
    fn edge_idx(&self, id: LinkId) -> Option<EdgeIndex<NodeId>> {
        self.inner.find_edge(id.from.node.into(), id.to.node.into())
    }

    fn edge_weight(&self, id: LinkId) -> Option<(EdgeIndex<NodeId>, &VectorGraphEdges)> {
        let id = self.edge_idx(id)?;
        return Some((id, self.inner.edge_weight(id)?));
    }

    fn edge_weight_mut(&mut self, id: LinkId) -> Option<(EdgeIndex<NodeId>, &mut VectorGraphEdges)> {
        let id = self.edge_idx(id)?;
        return Some((id, self.inner.edge_weight_mut(id)?));
    }
}

// graph trait impl
impl Graph for VectorGraph {
    #[inline]
    fn has_node(&self, id: NodeId) -> bool {
        self.inner.contains_node(id.into())
    }

    fn get_node(&self, id: NodeId) -> Option<NodeRef> {
        self.inner.node_weight(id.into())
            .map(|v| NodeRef::from(&**v))
    }

    #[inline]
    fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    fn has_link(&self, id: LinkId) -> bool {
        let pair = [id.from.socket, id.to.socket];

        match self.edge_weight(id) {
            Some((_, links)) => links.contains(&pair),
            None => false,
        }
    }

    #[inline]
    fn link_count(&self) -> usize {
        self.inner
            .graph()
            .edge_weights()
            .map(|edges| edges.len())
            .sum()
    }
}

impl GraphMut for VectorGraph {
    fn insert_node(&mut self, node: impl Into<Box<dyn Node>>) -> NodeId {
        NodeId(self.inner.add_node(node.into()).index() as u32)
    }

    #[inline]
    fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>> {
        self.inner.remove_node(id.into())
    }

    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut> {
        self.inner.node_weight_mut(id.into())
            .map(move |v| NodeMut::from(v))
    }

    fn insert_link(&mut self, id: LinkId) -> Result<(), WouldCycle> {
        let pair = [id.from.socket, id.to.socket];

        match self.edge_weight_mut(id) {
            // Edge set exists, append to it
            Some((_, links)) => {
                links.insert(pair);
                return Ok(());
            },

            // Edge doesn't exist, add it
            None => self.inner.add_edge(
                id.from.node.into(),
                id.to.node.into(),
                VectorGraphEdges::single(pair),
            ).map(|_| ()).map_err(|e| e.into()),
        }
    }

    fn remove_link(&mut self, id: LinkId) {
        let pair = [id.from.socket, id.to.socket];

        match self.edge_weight_mut(id) {
            Some((index, links)) => {
                // Remove the link from the set
                links.remove(&pair);

                // If the links set is zero, we remove it from the graph
                if links.len() == 0 { self.inner.remove_edge(index); }
            },

            // We don't have to do anything
            None => return,
        }
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

struct VectorGraphEdges {
    edges: SmallVec<[[SocketId; 2]; 4]>,
}

impl VectorGraphEdges {
    fn new() -> Self {
        Self {
            edges: SmallVec::new(),
        }
    }

    fn single(value: [SocketId; 2]) -> Self {
        let mut val = VectorGraphEdges::new();
        val.edges.push(value);

        return val;
    }

    // returns true if the link existed
    fn insert(&mut self, sockets: [SocketId; 2]) -> bool {
        match self.edges.binary_search(&sockets) {
            Ok(_) => return true,
            Err(index) => {
                self.edges.insert(index, sockets);
                return false;
            },
        }
    }

    // returns true if the link existed
    fn remove(&mut self, sockets: &[SocketId; 2]) -> bool {
        match self.edges.binary_search(sockets) {
            Ok(index) => {
                self.edges.remove(index);
                return true;
            }
            Err(_) => return false,
        }
    }

    // returns true if the link exists
    fn contains(&self, sockets: &[SocketId; 2]) -> bool {
        self.edges.binary_search(sockets).is_ok()
    }

    #[inline]
    fn len(&self) -> usize {
        self.edges.len()
    }
}