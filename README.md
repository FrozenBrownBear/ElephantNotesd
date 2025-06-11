# Notes App (Rust + egui)

Petit gestionnaire de notes avec barre latérale inspirée de macOS/iOS.

## Installation rapide

```bash
git clone https://github.com/votrecompte/notes_app.git
cd notes_app
./scripts/install.sh
```

Sur Windows (PowerShell) :

```powershell
./scripts/install.ps1
```

## Lancer l'application

```bash
cargo run
```

## Cross‑compilation

* Web : `rustup target add wasm32-unknown-unknown` puis `cargo build --target wasm32-unknown-unknown`
* Android/iOS : voir [tauri‑apps/tauri-mobile](https://github.com/tauri-apps/tauri-mobile)

## Architecture

* `models.rs` : structures métier (`Note`, `Folder`, etc.)
* `app.rs` : état global + machine à messages (pattern Elm)
* `ui/` : rendu avec egui, un fichier par composant
* `scripts/` : installateurs multi‑OS

Améliorez, fork‑ez, contribuez ! ✨
