use noodle_core::NodeId;

pub(crate) struct Visited(Vec<NodeId>);

impl Visited {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(amt: usize) -> Self {
        Self(Vec::with_capacity(amt))
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