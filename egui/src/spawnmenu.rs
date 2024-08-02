//! Node instantiation widgets.

use egui::{Style, Widget};
use noodle_core::Graph;

/// A menu for spawning nodes.
pub struct NodeSpawnMenuBuilder<'a, G> {
    graph: &'a G,
}

impl<'a, G> NodeSpawnMenuBuilder<'a, G>
where
    G: Graph,
{
    /// Creates a new [`NodeSpawnMenuBuilder`].
    pub fn new(graph: &'a mut G) -> Self {
        Self {
            graph,
        }
    }
}

impl<'a, G> Widget for NodeSpawnMenuBuilder<'a, G> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::canvas(&Style::default()).show(ui, |ui| {
            ui.label("Work in progress");
        }).response
    }
}