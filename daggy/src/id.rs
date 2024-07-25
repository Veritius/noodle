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