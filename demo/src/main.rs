//! A simple demo of a node application made with `noodle_core`, `noodle_egui`, and `eframe`. 

#![warn(missing_docs)]

mod app;
mod nodes;
mod settings;

fn main() -> eframe::Result {
    let app = app::DemoApp::default();
    app::run_app(app)
}