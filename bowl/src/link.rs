use core::fmt::Debug;

use noodle_core::SocketLinkId;
use smallvec::SmallVec;

/// A collection of links in a graph.
pub struct Links<EM> {
    edges: SmallVec<[Link<EM>; 1]>,
}

impl<EM> Links<EM> {
    pub(crate) fn new() -> Self {
        Links {
            edges: SmallVec::new(),
        }
    }

    pub(crate) fn insert(&mut self, id: SocketLinkId, meta: Option<EM>) {
        if let Err(idx) = self.edges.binary_search_by(|v| v.id.cmp(&id)) {
            self.edges.insert(idx, Link { id, meta });
        }
    }

    pub(crate) fn remove(&mut self, id: SocketLinkId) -> Option<EM> {
        let idx = self.edges.binary_search_by(|v| v.id.cmp(&id)).ok()?;
        return self.edges.remove(idx).meta;
    }

    /// Iterator over all links in the set.
    /// 
    /// This iterator is always in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = &Link<EM>> + '_ {
        self.edges.iter()
    }

    /// Borrows the [`Link`] of the given `id`, if it exists.
    pub fn get(&self, id: SocketLinkId) -> Option<&Link<EM>> {
        let idx = self.edges.binary_search_by(|v| v.id.cmp(&id)).ok()?;
        return Some(&self.edges[idx]);
    }

    /// Returns true if the link corresponding to `id` exists.
    pub fn contains(&self, id: SocketLinkId) -> bool {
        self.edges.binary_search_by(|v| v.id.cmp(&id)).is_ok()
    }

    /// Returns the number of links between the two nodes.
    #[inline]
    pub fn count(&self) -> usize {
        self.edges.len()
    }
}

impl<EM> Debug for Links<EM>
where
    EM: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set()
        .entries(self.iter())
        .finish()
    }
}

/// A link with associated metadata.
pub struct Link<EM> {
    /// The ID of the link.
    pub id: SocketLinkId,

    /// Associated metadata.
    pub meta: Option<EM>,
}

impl<EM> Debug for Link<EM>
where
    EM: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Link")
        .field("id", &self.id)
        .field("meta", &self.meta)
        .finish()
    }
}