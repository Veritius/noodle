use noodle_core::SocketId;
use smallvec::SmallVec;

pub(crate) struct VectorGraphEdges {
    edges: SmallVec<[[SocketId; 2]; 4]>,
}

impl VectorGraphEdges {
    pub fn new() -> Self {
        Self {
            edges: SmallVec::new(),
        }
    }

    pub fn single(value: [SocketId; 2]) -> Self {
        let mut val = VectorGraphEdges::new();
        val.edges.push(value);

        return val;
    }

    // returns true if the link existed
    pub fn insert(&mut self, sockets: [SocketId; 2]) -> bool {
        match self.edges.binary_search(&sockets) {
            Ok(_) => return true,
            Err(index) => {
                self.edges.insert(index, sockets);
                return false;
            },
        }
    }

    // returns true if the link existed
    pub fn remove(&mut self, sockets: &[SocketId; 2]) -> bool {
        match self.edges.binary_search(sockets) {
            Ok(index) => {
                self.edges.remove(index);
                return true;
            }
            Err(_) => return false,
        }
    }

    // returns true if the link exists
    pub fn contains(&self, sockets: &[SocketId; 2]) -> bool {
        self.edges.binary_search(sockets).is_ok()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.edges.len()
    }
}