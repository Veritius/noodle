//! Widgets for displaying [`Node`] objects in a graphical way.

use noodle_core::*;

/// A builder for a node view.
pub struct NodeViewBuilder<'a, N: Node> {
    node: &'a mut N,

    title: Option<&'a str>,
}

impl<'a, N: Node> NodeViewBuilder<'a, N> {
    /// Creates a new [`NodeViewBuilder`].
    pub fn new(node: &'a mut N) -> Self {
        Self {
            node,

            title: None,
        }
    }

    /// Use a specific name as the title of the node.
    /// If unset (`None`), the node's discriminator is used instead.
    /// 
    /// Defaults to `None` (the node's discriminator).
    pub fn title(mut self, name: Option<&'a str>) -> Self {
        self.title = name;
        return self;
    }
}