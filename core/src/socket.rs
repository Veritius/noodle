use core::marker::PhantomData;
use alloc::boxed::Box;
use super::*;

/// An ID for a socket belonging to one side of a [`Node`].
/// 
/// The inputs and outputs have unique ID sets.
/// Mixing them is not recommended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketId(pub u16);

/// The shape of the socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SocketShape {
    /// The socket cannot have any input.
    Disabled,

    /// The socket can have no input, or one input.
    Optional,

    /// The socket must have one and only one input.
    ExactlyOne,

    /// The socket must have at least one connection.
    AtLeastOne,

    /// The socket can have zero or more connections.
    Unlimited,
}

impl SocketShape {
    /// Returns true if `self` can connect to `other`.
    pub fn fits_into(&self, other: &Self) -> bool {
        use SocketShape::*;

        match (self, other) {
            // Disabled sockets never fit into other sockets
            (Disabled, _) => false,
            (_, Disabled) => false,

            // All sockets fit into Unlimited sockets
            (_, Unlimited) => true,

            (Optional, Optional) => true,
            (Optional, ExactlyOne) => false,
            (Optional, AtLeastOne) => false,

            (ExactlyOne, Optional) => true,
            (ExactlyOne, ExactlyOne) => true,
            (ExactlyOne, AtLeastOne) => true,

            (AtLeastOne, Optional) => true,
            (AtLeastOne, ExactlyOne) => true,
            (AtLeastOne, AtLeastOne) => true,

            (Unlimited, Optional) => false,
            (Unlimited, ExactlyOne) => false,
            (Unlimited, AtLeastOne) => false,
        }
    }
}

/// A socket controlled by a [`Node`](crate::Node).
pub struct Socket<'a> {
    /// The ID of the socket.
    pub id: SocketId,

    /// The shape of the socket.
    pub shape: SocketShape,

    #[doc(hidden)]
    pub phantom: PhantomData<&'a ()>,
}

/// An iterator over a set of [`Socket`] handles.
pub struct SocketIter<'a> {
    iter: Box<dyn Iterator<Item = Socket<'a>> + 'a>,
}

impl<'a> Iterator for SocketIter<'a> {
    type Item = Socket<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> From<Box<dyn Iterator<Item = Socket<'a>> + 'a>> for SocketIter<'a> {
    fn from(value: Box<dyn Iterator<Item = Socket<'a>> + 'a>) -> Self {
        Self { iter: value }
    }
}

/// A paired [`NodeId`] and [`SocketId`] for identifying sockets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeSocketId {
    /// The node the socket belongs to.
    pub node: NodeId,
    /// The socket belonging to the node.
    pub socket: SocketId,
}

/// A pair of [`NodeSocketId`], identifying a link between sockets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkId {
    /// The left socket.
    pub from: NodeSocketId,

    /// The right socket.
    pub to: NodeSocketId,
}

/// Returned when adding a link would form a cycle in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WouldCycle;

impl core::fmt::Display for WouldCycle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("would cycle")
    }
}

#[cfg(feature="std")]
impl std::error::Error for WouldCycle {}