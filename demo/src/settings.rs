use egui::{Direction, Widget};

pub(crate) struct AppSettings {
    pub graph_direction: Direction,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            graph_direction: Direction::LeftToRight,
        }
    }
}

impl Widget for &mut AppSettings {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Grid::new("settings_grid").show(ui, |ui| {
            ui.label("Layout");
            egui::ComboBox::new("settings_graph_dir", "")
            .selected_text(dir_string(self.graph_direction))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.graph_direction, Direction::LeftToRight, dir_string(Direction::LeftToRight));
                ui.selectable_value(&mut self.graph_direction, Direction::RightToLeft, dir_string(Direction::RightToLeft));
                ui.selectable_value(&mut self.graph_direction, Direction::TopDown, dir_string(Direction::TopDown));
                ui.selectable_value(&mut self.graph_direction, Direction::BottomUp, dir_string(Direction::BottomUp));
            });
            ui.end_row();
        }).response
    }
}

fn dir_string(dir: Direction) -> &'static str {
    match dir {
        Direction::LeftToRight => "Left to right",
        Direction::RightToLeft => "Right to left",
        Direction::TopDown => "Top to bottom",
        Direction::BottomUp => "Bottom to top",
    }
}