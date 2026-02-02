use clap::Parser;

#[derive(Parser)]
#[command(name = "csrc")]
pub struct Cli {
    #[arg(long, value_parser = parse_key_val, number_of_values = 1)]
    pub conf: Vec<(String, String)>,
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let (k, v) = s.split_once('=').ok_or("Expected KEY=VALUE")?;
    Ok((k.to_string(), v.to_string()))
}
