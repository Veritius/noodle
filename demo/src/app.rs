use eframe::egui;
use egui::Widget;
use noodle_bowl::HashGraph;
use noodle_egui::{graphview::GraphViewBuilder, spawnmenu::NodeSpawnMenuBuilder};
use crate::{nodes::LocalNode, settings::AppSettings};

pub(super) struct DemoApp {
    pub settings: AppSettings,
    show_settings: bool,

    pub graph: HashGraph<LocalNode>,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add_space(2.0);

            ui.horizontal(|ui |{
                ui.toggle_value(&mut self.show_settings, "Settings");
            });

            ui.add_space(2.0);
        });

        if self.show_settings {
            egui::SidePanel::left("settings")
            .resizable(false)
            .show(ctx, |ui| {
                self.settings.ui(ui);
            });
        }

        egui::SidePanel::right("spawn")
        .resizable(false)
        .show(ctx, |ui| {
            NodeSpawnMenuBuilder::new(&mut self.graph)
                .ui(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            GraphViewBuilder::<&_, LocalNode>::new(&self.graph)
                .direction(self.settings.graph_direction)
                .ui(ui)
        });
    }
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            settings: AppSettings::default(),
            show_settings: false,

            graph: HashGraph::default(),
        }
    }
}

pub(super) fn run_app(app: DemoApp) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "noodle demo",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}