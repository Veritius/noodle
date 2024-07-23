use petgraph::csr::IndexType;
use crate::NodeId;

unsafe impl IndexType for NodeId {
    #[inline(always)]
    fn new(x: usize) -> Self {
        Self(x as u32)
    }

    #[inline(always)]
    fn index(&self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    fn max() -> Self {
        Self(u32::MAX)
    }
}