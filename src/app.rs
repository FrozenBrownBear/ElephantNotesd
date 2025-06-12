use crate::models::{Folder, Note};
use crate::ui::sidebar::SideBar;
use eframe::egui;
use egui::{
    text::{LayoutJob, TextFormat},
    Color32, TextEdit, TextStyle, TextureHandle,
};
use egui_file_dialog::FileDialog;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, EventKind};
use pulldown_cmark::{html, Event, Parser, Tag};

#[derive(Clone)]
pub enum Msg {
    SelectFolder(usize),
    SelectHome,
    GoBack,
    OpenSettings,
    CreateItem, // bouton +
}

fn markdown_job(text: &str, style: &egui::Style) -> LayoutJob {
    let parser = Parser::new(text);
    let mut job = LayoutJob::default();
    let mut fmt = TextFormat {
        font_id: TextStyle::Body.resolve(style),
        color: style.visuals.text_color(),
        ..Default::default()
    };
    let mut stack: Vec<TextFormat> = Vec::new();
    let mut bullet_depth = 0usize;

    for ev in parser {
        match ev {
            Event::Start(Tag::Heading(level, _, _)) => {
                stack.push(fmt.clone());
                let mut id = TextStyle::Heading.resolve(style);
                let lvl: usize = level as usize;
                id.size = id.size - ((lvl.saturating_sub(1)) as f32) * 2.0;
                fmt.font_id = id;
            }
            Event::End(Tag::Heading(..)) => {
                fmt = stack.pop().unwrap_or(fmt);
                job.append("\n", 0.0, fmt.clone());
            }
            Event::Start(Tag::Emphasis) => {
                stack.push(fmt.clone());
                fmt.italics = true;
            }
            Event::End(Tag::Emphasis) => {
                fmt = stack.pop().unwrap_or(fmt);
            }
            Event::Start(Tag::Strong) => {
                stack.push(fmt.clone());
                fmt.color = style.visuals.strong_text_color();
            }
            Event::End(Tag::Strong) => {
                fmt = stack.pop().unwrap_or(fmt);
            }
            Event::Start(Tag::List(_)) => {
                bullet_depth += 1;
            }
            Event::End(Tag::List(_)) => {
                if bullet_depth > 0 { bullet_depth -= 1; }
                job.append("\n", 0.0, fmt.clone());
            }
            Event::Start(Tag::Item) => {
                for _ in 0..bullet_depth.saturating_sub(1) { job.append("  ", 0.0, fmt.clone()); }
                job.append("• ", 0.0, fmt.clone());
            }
            Event::End(Tag::Item) => {
                job.append("\n", 0.0, fmt.clone());
            }
            Event::Text(t) => job.append(&t, 0.0, fmt.clone()),
            Event::Code(t) => {
                let mut code_fmt = fmt.clone();
                code_fmt.font_id = TextStyle::Monospace.resolve(style);
                job.append(&t, 0.0, code_fmt);
            }
            Event::SoftBreak | Event::HardBreak => job.append("\n", 0.0, fmt.clone()),
            _ => {}
        }
    }
    job
}

#[derive(Clone)]
pub struct Icons {
    pub search: TextureHandle,
    pub back: TextureHandle,
    pub home: TextureHandle,
    pub settings: TextureHandle,
    pub add: TextureHandle,
}

impl Icons {
    fn load(ctx: &egui::Context) -> Self {
        fn png(bytes: &[u8], ctx: &egui::Context, id: &str) -> TextureHandle {
            let img = image::load_from_memory(bytes).unwrap().to_rgba8();
            let sz = [img.width() as usize, img.height() as usize];
            ctx.load_texture(
                id,
                egui::ColorImage::from_rgba_unmultiplied(sz, &img),
                egui::TextureOptions::LINEAR,
            )
        }
        Self {
            search:   png(include_bytes!("../assets/search.png"),   ctx, "icon_search"),
            back:     png(include_bytes!("../assets/back.png"),     ctx, "icon_back"),
            home:     png(include_bytes!("../assets/home.png"),     ctx, "icon_home"),
            settings: png(include_bytes!("../assets/setting.png"),  ctx, "icon_settings"),
            add:      png(include_bytes!("../assets/add.png"),      ctx, "icon_add"),
        }
    }
}

pub struct NotesApp {
    folders:           Vec<Folder>,
    selected:          Option<usize>,
    selected_note:     Option<usize>,
    show_settings:     bool,
    icons:             Option<Icons>,

    working_dir:          Option<PathBuf>,
    file_dialog:          FileDialog,
    dir_dialog_requested: bool,

    dark_mode: bool,

    watcher: Option<RecommendedWatcher>,
    rx: Option<Receiver<notify::Result<notify::Event>>>,
}

impl NotesApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            folders: Vec::new(),
            selected: None,
            selected_note: None,
            show_settings: false,
            icons: None,

            working_dir: None,
            file_dialog: FileDialog::new(),
            dir_dialog_requested: true,

            dark_mode: true,

            watcher: None,
            rx: None,
        }
    }

    fn watch(&mut self, path: &PathBuf) {
        if self.watcher.is_none() {
            let (tx, rx) = channel();
            let mut watcher = RecommendedWatcher::new(tx, Config::default()).ok();
            if let Some(w) = watcher.as_mut() {
                let _ = w.watch(path, RecursiveMode::NonRecursive);
            }
            self.watcher = watcher;
            self.rx = Some(rx);
        }
    }

    fn handle(&mut self, msg: Msg) {
        match msg {
            Msg::SelectFolder(i) => {
                self.selected = Some(i);
                self.selected_note = None;
                self.show_settings = false;
            }
            Msg::SelectHome => {
                self.selected = None;
                self.selected_note = None;
                self.show_settings = false;
            }
            Msg::GoBack => {
                if self.selected_note.is_some() {
                    self.selected_note = None;
                } else {
                    self.selected = None;
                }
                self.show_settings = false;
            }
            Msg::OpenSettings => {
                self.show_settings = true;
                self.selected = None;
                self.selected_note = None;
            }
            Msg::CreateItem => {
                if let Some(f_idx) = self.selected {
                    let name = format!("Nouvelle note {}", self.folders[f_idx].notes.len() + 1);
                    let path = self.folders[f_idx]
                        .path
                        .join(format!("note_{}.md", self.folders[f_idx].notes.len() + 1));
                    let _ = fs::write(&path, "");
                    self.folders[f_idx].notes.push(Note {
                        title: name.clone(),
                        body: String::new(),
                        path,
                    });
                    self.selected_note = Some(self.folders[f_idx].notes.len() - 1);
                } else if let Some(dir) = &self.working_dir {
                    let name = format!("Nouveau dossier {}", self.folders.len() + 1);
                    let path = dir.join(&name);
                    let _ = fs::create_dir_all(&path);
                    self.folders.push(Folder {
                        name,
                        color: Color32::from_rgb(100, 100, 200),
                        notes: Vec::new(),
                        path,
                    });
                }
            }
        }
    }
}

impl eframe::App for NotesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.icons.is_none() {
            self.icons = Some(Icons::load(ctx));
        }
        let icons = self.icons.as_ref().unwrap();

        // sélection du dossier au premier lancement
        if self.working_dir.is_none() {
            if self.dir_dialog_requested {
                self.file_dialog.pick_directory();
                self.dir_dialog_requested = false;
            }
            self.file_dialog.update(ctx);
            if let Some(path) = self.file_dialog.take_picked() {
                self.working_dir = Some(path);
            }
            return;
        }

        //------------------------------------------------------------------
        // Barre latérale
        //------------------------------------------------------------------
        let icon_px = (ctx.screen_rect().height() * 0.065).clamp(32.0, 64.0);
        let mut pending: Option<Msg> = None;

        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(icon_px + 12.0)
            .show(ctx, |ui| {
                pending = SideBar::new(
                    &self.folders,
                    self.selected,
                    self.show_settings,
                    icons,
                    icon_px,
                )
                .render(ui);
            });

        if let Some(msg) = pending {
            self.handle(msg);
        }

        //------------------------------------------------------------------
        // Panneau central
        //------------------------------------------------------------------
        egui::CentralPanel::default().show(ctx, |ui| {
            // 1) Paramètres
            if self.show_settings {
                ui.heading("Paramètres");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Mode sombre");
                    if ui.checkbox(&mut self.dark_mode, "").changed() {
                        ctx.set_visuals(if self.dark_mode {
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
                return;
            }

            // 2) Affichage / édition d’une note
            if let (Some(f_idx), Some(n_idx)) = (self.selected, self.selected_note) {
                {
                    let path = self.folders[f_idx].notes[n_idx].path.clone();
                    self.watch(&path);
                    let note = &mut self.folders[f_idx].notes[n_idx];
                    if let Some(rx) = &self.rx {
                        while let Ok(evt) = rx.try_recv() {
                            if let Ok(ev) = evt {
                                if matches!(ev.kind, EventKind::Modify(_)) {
                                    if let Ok(txt) = fs::read_to_string(&note.path) {
                                        note.body = txt;
                                    }
                                }
                            }
                        }
                    }

                    // édition du titre (single-line)
                    ui.add(TextEdit::singleline(&mut note.title).hint_text("Titre de la note"));
                    ui.add_space(8.0);

                    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                        let mut job = markdown_job(string, ui.style());
                        job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(job))
                    };

                    if ui
                        .add(
                            TextEdit::multiline(&mut note.body)
                                .desired_rows(20)
                                .layouter(&mut layouter)
                                .hint_text("Contenu…"),
                        )
                        .changed()
                    {
                        let _ = fs::write(&note.path, &note.body);
                        let parser = Parser::new(&note.body);
                        let mut html = String::new();
                        html::push_html(&mut html, parser);
                        let _ = fs::write(note.path.with_extension("html"), html);
                    }
                }

                return;
            }

            // 3) Liste des notes d’un dossier
            if let Some(f_idx) = self.selected {
                let folder = &mut self.folders[f_idx];
                ui.heading(&folder.name);
                ui.add_space(8.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (idx, note) in folder.notes.iter().enumerate() {
                        if ui
                            .selectable_label(false, &note.title)
                            .on_hover_text(&note.body)
                            .clicked()
                        {
                            self.selected_note = Some(idx);
                        }
                        ui.add_space(4.0);
                    }
                });
                return;
            }

            // 4) Écran d’accueil
            ui.heading("Bienvenue dans Notes App !");
            ui.label("Sélectionne un dossier ou crée-en un nouveau avec le bouton +.");
        });
    }
}
