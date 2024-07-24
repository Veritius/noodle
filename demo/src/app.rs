use eframe::egui;
use egui::Widget;
use noodle_core::VectorGraph;
use noodle_egui::graphview::GraphViewBuilder;
use crate::settings::AppSettings;

pub(super) struct DemoApp {
    pub settings: AppSettings,
    show_settings: bool,

    pub graph: VectorGraph,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.show_settings {
            egui::SidePanel::left("settings")
            .show(ctx, |ui| {
                self.settings.ui(ui);
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.checkbox(&mut self.show_settings, "Settings");
            });

            GraphViewBuilder::<&_>::new(&self.graph)
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

            graph: VectorGraph::new(),
        }
    }
}

pub(super) fn run_app(app: DemoApp) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "noodle demo",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}