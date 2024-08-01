use core::fmt::Debug;
use std::ops::{Deref, DerefMut};

/// A vertex in a graph.
pub struct Vertex<N, NM> {
    /// The internal node.
    pub node: N,

    /// The vertex metadata.
    pub meta: Option<NM>,
}

impl<N, NM> Deref for Vertex<N, NM> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<N, NM> DerefMut for Vertex<N, NM> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl<V, NM> Debug for Vertex<V, NM>
where
    V: Debug,
    NM: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Vertex")
        .field("node", &self.node)
        .field("meta", &self.meta)
        .finish()
    }
}