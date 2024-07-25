use daggy::NodeIndex;
use noodle_core::NodeId;
use petgraph::csr::IndexType;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub(crate) struct NodeIdWrap(pub NodeId);

unsafe impl IndexType for NodeIdWrap {
    #[inline(always)]
    fn new(x: usize) -> Self {
        Self(NodeId(x as u32))
    }

    #[inline(always)]
    fn index(&self) -> usize {
        self.0.0 as usize
    }

    #[inline(always)]
    fn max() -> Self {
        Self(NodeId(u32::MAX))
    }
}

impl From<NodeIndex<NodeIdWrap>> for NodeIdWrap {
    #[inline(always)]
    fn from(value: NodeIndex<NodeIdWrap>) -> Self {
        value.into()
    }
}

impl From<NodeId> for NodeIdWrap {
    #[inline(always)]
    fn from(value: NodeId) -> Self {
        Self(value)
    }
}

impl From<NodeIdWrap> for NodeId {
    #[inline(always)]
    fn from(value: NodeIdWrap) -> Self {
        value.0
    }
}

#[inline]
pub(crate) fn node_index_to_node_id(index: NodeIndex<NodeIdWrap>) -> NodeId {
    NodeIdWrap::from(index).into()
}

#[inline]
pub(crate) fn node_id_to_node_index(id: NodeId) -> NodeIndex<NodeIdWrap> {
    NodeIndex::from(NodeIdWrap::from(id))
}