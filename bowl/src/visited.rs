use noodle_core::NodeId;
use smallvec::SmallVec;

pub(crate) struct Visited(SmallVec<[NodeId; 4]>);

impl Visited {
    pub fn new() -> Self {
        Self(SmallVec::new())
    }

    pub fn with_capacity(amt: usize) -> Self {
        Self(SmallVec::with_capacity(amt))
    }

    pub fn visit(&mut self, id: NodeId) -> bool {
        match self.0.binary_search(&id) {
            Ok(_) => { return false },

            Err(idx) => {
                self.0.insert(idx, id);
                return true;
            },
        }
    }

    #[inline]
    pub fn is_visited(&self, id: NodeId) -> bool {
        self.0.binary_search(&id).is_ok()
    }
}