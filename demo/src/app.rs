use eframe::egui;
use egui::Widget;
use noodle_daggy::UncachedGraph;
use noodle_egui::graphview::GraphViewBuilder;
use crate::settings::AppSettings;

pub(super) struct DemoApp {
    pub settings: AppSettings,
    show_settings: bool,

    pub graph: UncachedGraph,
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
            .show(ctx, |ui| {
                self.settings.ui(ui);
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
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

            graph: UncachedGraph::new(),
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