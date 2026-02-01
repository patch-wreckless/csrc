mod conf;

fn main() {
    if let Err(e) = run() {
        eprintln!("fatal: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = conf::read_config_from_file()?;
    println!("sourceRoot: {}", config.source_root);
    Ok(())
}
