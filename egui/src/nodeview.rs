//! Widgets for displaying [`Node`] objects in a graphical way.

use egui::Widget;
use noodle_core::*;

/// A builder for a node view.
/// 
/// Can be constructed from any of the following types:
/// - [`NodeRef`] - Type-erased immutable access
/// - [`NodeMut`] - Type-erased mutable access
/// - `&N where N: Node` - Typed immutable access
/// - `&mut N where N: Node` - Typed mutable access
pub struct NodeViewBuilder<'a, N: 'a> {
    node: N,

    title: Option<&'a str>,
}

impl<'a, N> NodeViewBuilder<'a, N> {
    fn new_inner(node: N) -> Self {
        Self {
            node,

            title: None,
        }
    }
}

impl<'a> NodeViewBuilder<'a, NodeRef<'a>> {
    /// Creates a new [`NodeViewBuilder`].
    #[inline]
    pub fn new(node_ref: NodeRef<'a>) -> Self {
        Self::new_inner(node_ref)
    }
}

impl<'a> NodeViewBuilder<'a, NodeMut<'a>> {
    /// Creates a new [`NodeViewBuilder`].
    #[inline]
    pub fn new(node_mut: NodeMut<'a>) -> Self {
        Self::new_inner(node_mut)
    }
}

impl<'a, N: Node> NodeViewBuilder<'a, &'a N> {
    /// Creates a new [`NodeViewBuilder`].
    #[inline]
    pub fn new(node: &'a N) -> Self {
        Self::new_inner(node)
    }
}

impl<'a, N: Node> NodeViewBuilder<'a, &'a mut N> {
    /// Creates a new [`NodeViewBuilder`].
    #[inline]
    pub fn new(node: &'a mut N) -> Self {
        Self::new_inner(node)
    }
}

impl<'a, N> NodeViewBuilder<'a, N> {
    /// Use a specific name as the title of the node.
    /// If unset (`None`), the node's discriminator is used instead.
    /// 
    /// Defaults to `None` (the node's discriminator).
    pub fn title(mut self, name: Option<&'a str>) -> Self {
        self.title = name;
        return self;
    }
}

impl<'a> Widget for NodeViewBuilder<'a, NodeRef<'a>> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        todo!()
    }
}

impl<'a> Widget for NodeViewBuilder<'a, NodeMut<'a>> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        todo!()
    }
}

impl<'a, N: Node> Widget for NodeViewBuilder<'a, &'a N> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        todo!()
    }
}

impl<'a, N: Node> Widget for NodeViewBuilder<'a, &'a mut N> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        todo!()
    }
}