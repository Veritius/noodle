//! Widgets for displaying [`Node`] objects in a graphical way.

use noodle_core::*;

/// A builder for a node view.
pub struct NodeViewBuilder<'a> {
    node: &'a mut dyn Node,
}