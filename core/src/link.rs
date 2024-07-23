use core::marker::PhantomData;
use super::*;

/// A paired [`NodeId`] and [`SocketId`] for identifying sockets.
pub struct LinkId {
    /// The node the socket belongs to.
    pub node: NodeId,
    /// The socket belonging to the node.
    pub socket: SocketId,
}

/// A link between two sockets.
pub struct Link<'a> {
    /// The socket the first end of the link is connected to.
    pub from: SocketId,

    /// The socket the second end of the link is connected to.
    pub to: SocketId,

    /// The type of data the link carries.
    pub vtype: ValueTypeId,

    #[doc(hidden)]
    pub phantom: PhantomData<&'a ()>,
}