use egui::{self, Ui};
use crate::models::Folder;

pub struct NotesPanel<'a> {
    folders: &'a [Folder],
    current: Option<usize>,
}

impl<'a> NotesPanel<'a> {
    pub fn new(folders: &'a [Folder], current: Option<usize>) -> Self {
        Self { folders, current }
    }

    pub fn render(&self, ui: &mut Ui) {
        let heading = match self.current {
            Some(idx) => &self.folders[idx].name,
            None => "Notes",
        };
        ui.heading(heading);
        ui.add_space(12.0);

        if let Some(idx) = self.current {
            list_notes(ui, &self.folders[idx]);
        } else {
            ui.colored_label(
                egui::Color32::GRAY,
                "Sélectionnez un dossier dans la barre latérale.",
            );
        }
    }
}

fn list_notes(ui: &mut Ui, folder: &Folder) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        for note in &folder.notes {
            ui.group(|ui| {
                ui.label(&note.title);
                ui.label(&note.body);
            });
            ui.add_space(8.0);
        }
    });
}
