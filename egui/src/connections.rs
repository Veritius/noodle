//! Connection display.

/// Style configuration for lines representing graph links.
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct LinkStyle {
    /// The line pattern.
    pub pattern: Pattern,

    /// The curvature of the line.
    pub curvature: Curvature,

    /// The radius of a 'glow' effect, used to highlight the line.
    pub glow_radius: f32,
}

/// A pattern applied to the line.
/// 
/// Primarily to visually distinguish a link from other links of the same type.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Pattern {
    /// A solid, unbroken line.
    /// 
    /// `―――――――――――`
    #[default]
    Solid,

    /// A solid, occasionally broken line.
    /// 
    /// `―― ―― ―― ――`
    Dashed,
}

/// The 'curvature' of the line.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Curvature {
    /// A straight line drawn from the start point to the end point.
    Straight,

    /// A line that smoothly curves between its start and end point.
    #[default]
    Curved,
}

/// Style configuration for shapes representing node sockets.
#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct SocketStyle {
    /// The shape of the socket.
    pub shape: Shape,
}

/// The shape of a socket.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Shape {
    /// A simple, perfect circle.
    #[default]
    Circle,

    /// A simple square with rounded edges.
    Square,

    /// A diamond shape. Looks like a square rotated by 45 degrees.
    Diamond,
}