use egui::{self, Ui, Color32, Context};

pub struct SettingsPanel<'a> {
    dark_mode: &'a mut bool,
}

impl<'a> SettingsPanel<'a> {
    pub fn new(dark_mode: &'a mut bool) -> Self {
        Self { dark_mode }
    }

    pub fn render(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.heading("Paramètres");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Mode sombre");
            if ui.checkbox(self.dark_mode, "").changed() {
                ctx.set_visuals(if *self.dark_mode {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                });
            }
        });

        ui.separator();
        ui.label(
            egui::RichText::new("Version 0.1 – Demo")
                .color(Color32::GRAY)
                .small(),
        );
    }
}
