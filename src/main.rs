mod conf;

fn main() {
    let (config_file_path, ok) = conf::get_config_file_path();
    println!(
        "configuration file path: {}\nfound: {}",
        config_file_path.display(),
        ok
    );
}
