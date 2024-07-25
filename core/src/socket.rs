use core::marker::PhantomData;
use std::any::TypeId;
use super::*;

/// An ID for a socket belonging to one side of a [`Node`].
/// 
/// The inputs and outputs have unique ID sets.
/// Mixing them is not recommended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketId(pub u16);

/// The shape of the socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Debug, Clone, Copy, Hash)]
pub struct SocketRef<'a> {
    /// The ID of the socket.
    pub id: SocketId,

    /// The shape of the socket.
    pub shape: SocketShape,

    /// The data type the socket accepts.
    pub vtype: TypeId,

    #[doc(hidden)]
    pub phantom: PhantomData<&'a ()>,
}

/// A set of sockets.
/// 
/// A `SocketSet` must be ordered by its `id` in ascending order and have no duplicate items.
#[derive(Clone, Copy, Hash)]
pub struct SocketSet<'a>(&'a [SocketRef<'a>]);

impl<'a> SocketSet<'a> {
    /// Try to create a new [`SocketSet`], checking if the slice is valid.
    pub fn new(slice: &'a [SocketRef<'a>]) -> Result<Self, InvalidSocketSet> {
        // Simultaneously checks that the set is both in order and has no duplicates
        if !slice.windows(2).all(|w| w[0].id < w[1].id) { return Err(InvalidSocketSet); }
        return Ok(unsafe { Self::new_unchecked(slice) });
    }

    /// Create a new [`SocketSet`] from a slice, without checking that it's valid.
    pub const unsafe fn new_unchecked(slice: &'a [SocketRef<'a>]) -> Self {
        Self(slice)
    }

    /// Gets the [`SocketRef`] for a given [`SocketId`], if present in the set.
    pub fn get(&self, id: SocketId) -> Option<SocketRef> {
        let idx = self.0.binary_search_by(|v| v.id.cmp(&id)).ok()?;
        return Some(self.0[idx]);
    }
}

impl<'a> TryFrom<&'a [SocketRef<'a>]> for SocketSet<'a> {
    type Error = InvalidSocketSet;

    #[inline(always)]
    fn try_from(slice: &'a [SocketRef<'a>]) -> Result<Self, Self::Error> {
        SocketSet::new(slice)
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

/// Error returned when the slice used to create a [`SocketSet`] did not satisfy the conditions.
#[derive(Debug, Clone, Copy)]
pub struct InvalidSocketSet;