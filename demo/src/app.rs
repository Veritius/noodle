use eframe::egui;
use egui::Widget;
use noodle_core::VectorGraph;
use noodle_egui::graphview::GraphViewBuilder;

pub(super) struct DemoApp {
    pub graph: VectorGraph,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            GraphViewBuilder::<&_>::new(&self.graph).ui(ui)
        });
    }
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
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