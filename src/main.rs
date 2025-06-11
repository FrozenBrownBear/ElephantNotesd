use eframe::{egui, NativeOptions};

mod app;
mod ui;
mod models;

fn main() -> eframe::Result<()> {
    let native = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(800.0, 600.0)), // fenêtre plus grande
        ..Default::default()
    };

    eframe::run_native(
        "Notes App",
        native,
        Box::new(|cc| Ok(Box::new(app::NotesApp::new(cc)))),
    )
}
