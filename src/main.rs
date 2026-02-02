use std::path::{Path, PathBuf};

use clap::Parser;

mod cli;
mod conf;
mod fzf;
mod scan;

fn main() {
    if let Err(e) = run() {
        eprintln!("fatal: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::try_parse()?;
    let config = conf::load_config(&cli.conf)?;

    let source_root = shellexpand::full(&config.source_root.0.to_string_lossy())
        .map(|e| Path::new(e.as_ref()).to_path_buf())?;

    let source_directories = scan::get_source_directories(&source_root)?
        .into_iter()
        .filter_map(|p| format_path(&p));

    if let Some(selected) = fzf::select_directory(source_directories)? {
        println!("{}", selected);
    }

    Ok(())
}

fn format_path(p: &PathBuf) -> Option<String> {
    let base = p.file_name().map(|s| s.to_str())??;
    Some(format!("{base} {}", p.to_string_lossy()))
}
