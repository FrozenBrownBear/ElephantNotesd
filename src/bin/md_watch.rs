use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use pulldown_cmark::{html, Options, Parser};
use std::path::Path;
use std::sync::mpsc::channel;
use std::{env, fs};

fn main() -> notify::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: md_watch <file>");
        std::process::exit(1);
    }
    let path = args[1].clone();
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(Path::new(&path), RecursiveMode::NonRecursive)?;

    if let Err(e) = convert(&path) {
        eprintln!("{e}");
    }

    for res in rx {
        match res {
            Ok(event) => match event.kind {
                EventKind::Modify(_) | EventKind::Create(_) => {
                    if let Err(e) = convert(&path) {
                        eprintln!("{e}");
                    }
                }
                _ => {}
            },
            Err(e) => eprintln!("watch error: {e}"),
        }
    }
    Ok(())
}

fn convert(path: &str) -> std::io::Result<()> {
    let md = fs::read_to_string(path)?;
    let parser = Parser::new_ext(&md, Options::all());
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);
    let html_path = format!("{}.html", path.trim_end_matches(".md"));
    fs::write(&html_path, html_out)?;
    println!("Updated {html_path}");
    Ok(())
}
