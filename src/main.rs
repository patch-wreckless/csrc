use clap::Parser;

mod cli;
mod conf;

fn main() {
    if let Err(e) = run() {
        eprintln!("fatal: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::try_parse()?;
    let config = conf::load_config(&cli.conf)?;
    println!("config: {:?}", config);
    Ok(())
}
