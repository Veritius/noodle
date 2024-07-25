use core::ops::{Deref, DerefMut};
use super::*;

/// A unique identifier for a [`Node`] within a [`Graph`].
/// 
/// A `NodeId` is only unique to the `Graph` that its `Node` exists in.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub u32);

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

impl<'a> From<&'a mut Box<dyn Node>> for NodeMut<'a> {
    fn from(value: &'a mut Box<dyn Node>) -> Self {
        Self { inner: value.as_mut() }
    }
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

/// A type that can produce new [`Node`] objects.
pub trait NodeFactory {
    /// The [discriminator](Node::discriminator) of the resulting [`Node`].
    fn discriminator(&self) -> &str;

    /// Creates a new node.
    fn new(&self) -> Box<dyn Node>;
}