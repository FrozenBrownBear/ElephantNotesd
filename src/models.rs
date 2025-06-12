use serde::{Serialize, Deserialize};
use egui::Color32;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    pub title: String,
    pub body: String,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Folder {
    pub name: String,
    pub color: Color32,
    pub notes: Vec<Note>,
    pub path: PathBuf,
}

impl Folder {
    pub fn new(name: &str, color: Color32, path: PathBuf) -> Self {
        Self {
            name: name.to_owned(),
            color,
            notes: Vec::new(),
            path,
        }
    }
}
