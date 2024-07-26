use core::{fmt::Debug, ops::{Deref, DerefMut}};
use super::*;

/// A unique identifier for a [`Node`] within a [`Graph`].
/// 
/// A `NodeId` is only unique to the `Graph` that its `Node` exists in.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NodeId(pub u32);

impl Debug for NodeId {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// A node type.
pub trait Node {
    /// Returns a `str` that identifies the node type.
    /// 
    /// To identify an instance of a node, use [`NodeId`].
    fn discriminator(&self) -> &str;

    /// Returns the sockets used for input values.
    fn input_sockets(&self) -> SocketSet;

    /// Returns the sockets used for output values.
    fn output_sockets(&self) -> SocketSet;

    /// 'Executes' the node, returning the output if successful.
    fn execute(
        &self,
        values: SocketValues,
        mask: OutputMask,
    ) -> Result<SocketValues, NodeExecutionError>;
}

/// A reference to a [`Node`] object.
pub struct NodeRef<'a, N: Node + 'a> {
    inner: &'a N,
}

impl<'a, N: Node> From<&'a N> for NodeRef<'a, N> {
    fn from(value: &'a N) -> NodeRef<'a, N> {
        Self { inner: value }
    }
}

impl<'a, N: Node + 'a> Deref for NodeRef<'a, N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

/// A mutable reference to a [`Node`] object.
pub struct NodeMut<'a, N: Node + 'a> {
    inner: &'a mut N,
}

impl<'a, N: Node> From<&'a mut N> for NodeMut<'a, N> {
    fn from(value: &'a mut N) -> Self {
        Self { inner: value }
    }
}

impl<'a, N: Node> Deref for NodeMut<'a, N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<N: Node> DerefMut for NodeMut<'_, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, N: Node> From<NodeMut<'a, N>> for NodeRef<'a, N> {
    fn from(value: NodeMut<'a, N>) -> Self {
        Self { inner: value.inner }
    }
}