use egui::{self, vec2, Color32, Image, ImageButton, Ui};
use egui::epaint::StrokeKind;
use crate::app::{Icons, Msg};
use crate::models::Folder;

pub struct SideBar<'a> {
    folders: &'a [Folder],
    current: Option<usize>,
    show_settings: bool,
    icons: &'a Icons,
    size: f32,
}

impl<'a> SideBar<'a> {
    pub fn new(
        folders: &'a [Folder],
        current: Option<usize>,
        show_settings: bool,
        icons: &'a Icons,
        size: f32,
    ) -> Self {
        Self {
            folders,
            current,
            show_settings,
            icons,
            size,
        }
    }

    pub fn render(&self, ui: &mut Ui) -> Option<Msg> {
        ui.add_space(self.size * 0.3);

        if icon(ui, &self.icons.search, self.size).clicked() {
            // recherche future
        }
        ui.add_space(self.size * 0.5);

        for (idx, folder) in self.folders.iter().enumerate() {
            if folder_button(ui, folder, self.current == Some(idx), self.size).clicked() {
                return Some(Msg::SelectFolder(idx));
            }
            ui.add_space(self.size * 0.25);
        }
        ui.add_space(self.size * 0.5);

        if icon(ui, &self.icons.back, self.size).clicked() {
            return Some(Msg::GoBack);
        }
        ui.add_space(self.size * 0.5);

        if icon(ui, &self.icons.add, self.size).clicked() {
            return Some(Msg::CreateItem);       // bouton +
        }
        ui.add_space(self.size * 0.5);

        if icon(ui, &self.icons.home, self.size).clicked() {
            return Some(Msg::SelectHome);
        }
        ui.add_space(self.size * 0.5);

        if icon(ui, &self.icons.settings, self.size)
            .on_hover_text(if self.show_settings { "Paramètres (ouvert)" } else { "Paramètres" })
            .clicked()
        {
            return Some(Msg::OpenSettings);
        }

        None
    }
}

fn icon(ui: &mut Ui, tex: &egui::TextureHandle, size: f32) -> egui::Response {
    // on supprime le tint → icônes noires (couleurs d’origine inversées)
    let image = Image::from_texture(tex).fit_to_exact_size(vec2(size, size));
    ui.add(ImageButton::new(image).frame(false))
}

fn folder_button(ui: &mut Ui, folder: &Folder, selected: bool, size: f32) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), egui::Sense::click());
    let painter = ui.painter();
    painter.rect_filled(rect, 4.0, folder.color);
    if selected {
        painter.rect_stroke(
            rect,
            4.0,
            egui::Stroke::new(2.0, Color32::WHITE),
            StrokeKind::Middle,
        );
    }
    resp
}
