mod conf;

fn main() {
    if let Err(e) = run() {
        eprintln!("fatal: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = conf::load_config()?;
    println!("config: {:?}", config);
    Ok(())
}
