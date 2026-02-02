use std::path::Path;

use clap::Parser;

mod cli;
mod conf;
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

    let source_directories = scan::get_source_directories(&source_root)?;
    println!("source directories: {:?}", source_directories);
    Ok(())
}
