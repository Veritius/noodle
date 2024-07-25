use noodle_core::*;
use daggy::{stable_dag::StableDag, EdgeIndex};
use crate::{edges::VectorGraphEdges, id::{node_id_to_node_index, NodeIdWrap}};

type GraphInner = StableDag<Box<dyn Node>, VectorGraphEdges, NodeIdWrap>;

/// A [`Graph`] implementation based on `daggy`'s [`StableDag`] type.
pub struct SimpleGraph {
    inner: GraphInner,
}

// public interface
impl SimpleGraph {
    /// Creates a new, empty [`UncachedGraph`].
    pub fn new() -> Self {
        Self {
            inner: GraphInner::new(),
        }
    }
}

// internal stuff
impl SimpleGraph {
    fn edge_idx(&self, id: LinkId) -> Option<EdgeIndex<NodeIdWrap>> {
        self.inner.find_edge(
            node_id_to_node_index(id.from.node),
            node_id_to_node_index(id.to.node)
        )
    }

    fn edge_weight(&self, id: LinkId) -> Option<(EdgeIndex<NodeIdWrap>, &VectorGraphEdges)> {
        let id = self.edge_idx(id)?;
        return Some((id, self.inner.edge_weight(id)?));
    }

    fn edge_weight_mut(&mut self, id: LinkId) -> Option<(EdgeIndex<NodeIdWrap>, &mut VectorGraphEdges)> {
        let id = self.edge_idx(id)?;
        return Some((id, self.inner.edge_weight_mut(id)?));
    }
}

// graph trait impl
impl Graph for SimpleGraph {
    fn insert_node(&mut self, node: impl Into<Box<dyn Node>>) -> NodeId {
        NodeId(self.inner.add_node(node.into()).index() as u32)
    }

    #[inline]
    fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>> {
        self.inner.remove_node(node_id_to_node_index(id))
    }

    #[inline]
    fn has_node(&self, id: NodeId) -> bool {
        self.inner.contains_node(node_id_to_node_index(id))
    }

    fn get_node(&self, id: NodeId) -> Option<NodeRef> {
        self.inner.node_weight(node_id_to_node_index(id))
            .map(|v| NodeRef::from(&**v))
    }

    fn get_node_mut(&mut self, id: NodeId) -> Option<NodeMut> {
        self.inner.node_weight_mut(node_id_to_node_index(id))
            .map(move |v| NodeMut::from(v))
    }

    #[inline]
    fn node_count(&self) -> Option<usize> {
        Some(self.inner.node_count())
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
                node_id_to_node_index(id.from.node),
                node_id_to_node_index(id.to.node),
                VectorGraphEdges::single(pair),
            ).map(|_| ()).map_err(|_| WouldCycle),
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

    fn has_link(&self, id: LinkId) -> bool {
        let pair = [id.from.socket, id.to.socket];

        match self.edge_weight(id) {
            Some((_, links)) => links.contains(&pair),
            None => false,
        }
    }

    #[inline]
    fn link_count(&self) -> Option<usize> {
        let v = self.inner
            .graph()
            .edge_weights()
            .map(|edges| edges.len())
            .sum();

        return Some(v);
    }

    // These don't do anything
    fn reserve_nodes(&mut self, _amt: usize) {}
    fn reserve_nodes_exact(&mut self, _amt: usize) {}
    fn reserve_links(&mut self, _amt: usize) {}
    fn reserve_links_exact(&mut self, _amt: usize) {}

    fn solve_node(
        &mut self,
        node: NodeId,
        outputs: OutputMask,
    ) -> Result<SocketValues, ()> {
        todo!()
    }
}