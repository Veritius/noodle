use core::ops::{Deref, DerefMut};

/// A generator for [`NodeId`] values.
pub struct NodeIdGenerator {
    #[cfg(not(feature="use_atomics"))]
    idx: usize,

    #[cfg(feature="use_atomics")]
    idx: portable_atomic::AtomicUsize,
}

impl Default for NodeIdGenerator {
    fn default() -> Self {
        Self {
            #[cfg(not(feature="use_atomics"))]
            idx: usize::default(),

            #[cfg(feature="use_atomics")]
            idx: portable_atomic::AtomicUsize::default(),
        }
    }
}

impl NodeIdGenerator {
    /// Generate a new [`NodeId`] for use.
    pub fn next(&mut self) -> NodeId {
        #[cfg(not(feature="use_atomics"))] {
            let v = self.idx;
            self.idx += 1;
            return NodeId(v);
        }

        #[cfg(feature="use_atomics")] {
            let v = *self.idx.get_mut();
            self.idx.get_mut();
            return NodeId(v);
        }
    }

    /// Generate a new [`NodeId`] for use using atomic operations.
    /// Since this uses atomic operations, this can be done with an immutable borrow.
    #[cfg(feature="use_atomics")]
    pub fn next_atomic(&self) -> NodeId {
        use core::sync::atomic::Ordering;
        let v = self.idx.fetch_add(1, Ordering::AcqRel);
        return NodeId(v);
    }
}

/// An identifier for a [`Node`].
/// Unique only to ids generated with the same [`NodeIdGenerator`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(usize);

/// A node type.
pub trait Node {

}

/// A reference to a [`Node`] object.
pub struct NodeRef<'a> {
    inner: &'a dyn Node,
}

impl<'a> From<&'a dyn Node> for NodeRef<'a> {
    fn from(value: &'a dyn Node) -> NodeRef<'a> {
        Self { inner: value }
    }
}

impl<'a> Deref for NodeRef<'a> {
    type Target = dyn Node + 'a;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

/// A mutable reference to a [`Node`] object.
pub struct NodeMut<'a> {
    inner: &'a mut dyn Node,
}

impl<'a> From<&'a mut dyn Node> for NodeMut<'a> {
    fn from(value: &'a mut dyn Node) -> NodeMut<'a> {
        Self { inner: value }
    }
}

impl<'a> Deref for NodeMut<'a> {
    type Target = dyn Node + 'a;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl DerefMut for NodeMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a> From<NodeMut<'a>> for NodeRef<'a> {
    fn from(value: NodeMut<'a>) -> Self {
        Self { inner: value.inner }
    }
}