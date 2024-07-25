use core::{marker::PhantomData, any::TypeId};
use super::*;

pub use sorted::*;

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
/// This set is created from a slice that is sorted by its [`SocketId`] in ascending order, and contains no duplicate values.
#[derive(Clone, Copy, Hash)]
pub struct SocketSet<'a>(SortedUniqueSlice<'a, SocketRef<'a>>);

impl<'a> SocketSet<'a> {
    /// Try to create a new [`SocketSet`], checking if the slice is valid.
    pub fn new(slice: &'a [SocketRef<'a>]) -> Result<Self, SortedUniqueSliceError> {
        SortedUniqueSlice::new(slice, |a,b| a.id.cmp(&b.id))
            .map(|v| Self(v))
    }

    /// Create a new [`SocketSet`] from a slice, without checking that it's valid.
    pub const unsafe fn new_unchecked(slice: &'a [SocketRef<'a>]) -> Self {
        Self(SortedUniqueSlice::new_unchecked(slice))
    }

    /// Gets the [`SocketRef`] for a given [`SocketId`], if present in the set.
    pub fn get(&self, id: SocketId) -> Option<SocketRef> {
        self.0.search(|v| v.id.cmp(&id)).cloned()
    }
}

impl<'a> TryFrom<&'a [SocketRef<'a>]> for SocketSet<'a> {
    type Error = SortedUniqueSliceError;

    #[inline(always)]
    fn try_from(slice: &'a [SocketRef<'a>]) -> Result<Self, Self::Error> {
        SocketSet::new(slice)
    }
}

/// A set of [`SocketId`] values that define a 'mask' of outputs that must be resolved.
/// This is useful if calculating an output value is expensive, and lets a [`Node`] avoid calculating it.
/// 
/// This set is created from a slice that is sorted by its [`SocketId`] in ascending order, and contains no duplicate values.
#[derive(Clone, Copy, Hash)]
pub struct OutputMask<'a>(SortedUniqueSlice<'a, SocketId>);

impl<'a> OutputMask<'a> {
    /// Try to create a new [`OutputMask`], checking if the slice is valid.
    pub fn new(slice: &'a [SocketId]) -> Result<Self, SortedUniqueSliceError> {
        SortedUniqueSlice::new(slice, |a,b| a.cmp(&b))
            .map(|v| Self(v))
    }

    /// Create a new [`OutputMask`] from a slice, without checking that it's valid.
    pub const unsafe fn new_unchecked(slice: &'a [SocketId]) -> Self {
        Self(SortedUniqueSlice::new_unchecked(slice))
    }

    /// Returns `true` if the given [`SocketId`] is **included** by the mask.
    pub fn includes(&self, id: SocketId) -> bool {
        self.0.search(|v| v.cmp(&id)).is_some()
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

mod sorted {
    //! [`SortedUniqueSlice`] gets its own module so that its internals are not visible.
    //! This allows us to make confident guarantees that it follows its conditions.

    use core::{cmp::Ordering, hash::Hash};

    #[derive(Clone, Copy)]
    pub(super) struct SortedUniqueSlice<'a, T: 'a>(&'a [T]);

    impl<'a, T: 'a> SortedUniqueSlice<'a, T> {
        pub fn new(slice: &'a [T], cmp: impl Fn(&T, &T) -> Ordering) -> Result<Self, SortedUniqueSliceError> {
            let mut iter = slice.windows(2);
            while let Some([a, b]) = iter.next() {
                match cmp(a,b) {
                    Ordering::Greater => { /* Do nothing */ },
                    Ordering::Less => return Err(SortedUniqueSliceError::ImproperOrdering),
                    Ordering::Equal => return Err(SortedUniqueSliceError::ContainsDuplicate),
                }
            }

            return Ok(Self(slice));
        }

        // SAFETY: The slice must be sorted in ascending order and not contain duplicates
        pub const unsafe fn new_unchecked(slice: &'a [T]) -> Self {
            Self(slice)
        }

        pub fn search(&self, bs: impl FnMut(&T) -> Ordering) -> Option<&T> {
            let idx = self.0.binary_search_by(bs).ok()?;
            return Some(&self.0[idx]);
        }
    }

    impl<T> Hash for SortedUniqueSlice<'_, T> where T: Hash {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state)
        }
    }

    /// An error that occurred while checking a slice was correctly ordered and unique.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SortedUniqueSliceError {
        /// The values in the slice were not in ascending order.
        ImproperOrdering,

        /// The slice contained duplicate values.
        ContainsDuplicate,
    }

    impl core::fmt::Display for SortedUniqueSliceError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(match self {
                SortedUniqueSliceError::ImproperOrdering => "improper ordering",
                SortedUniqueSliceError::ContainsDuplicate => "contains duplicate",
            })
        }
    }

    #[cfg(feature="std")]
    impl std::error::Error for SortedUniqueSliceError {}
}