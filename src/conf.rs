use std::{env, fmt, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "sourceRoot")]
    pub source_root: String,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound,
    ReadError(std::io::Error),
    ParseError(serde_yaml::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound => write!(f, "config file not found"),
            ConfigError::ReadError(e) => write!(f, "failed to read config file: {}", e),
            ConfigError::ParseError(e) => write!(f, "failed to parse config file: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::ReadError(e) => Some(e),
            ConfigError::ParseError(e) => Some(e),
            ConfigError::FileNotFound => None,
        }
    }
}

pub fn read_config_from_file() -> Result<Config, ConfigError> {
    let (config_file_path, ok) = get_config_file_path();
    if !ok {
        return Err(ConfigError::FileNotFound);
    }

    let contents =
        std::fs::read_to_string(config_file_path).map_err(|e| ConfigError::ReadError(e))?;

    let config: Config = serde_yaml::from_str(&contents).map_err(ConfigError::ParseError)?;

    Ok(config)
}

pub fn get_config_file_path() -> (PathBuf, bool) {
    let mut candidates = Vec::with_capacity(5);

    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        candidates.push(PathBuf::from(&xdg).join("csrc/config.yaml"));
        candidates.push(PathBuf::from(&xdg).join("csrc.yaml"));
    }

    if let Ok(home) = env::var("HOME") {
        candidates.push(PathBuf::from(&home).join(".config/csrc/config.yaml"));
        candidates.push(PathBuf::from(&home).join(".config/csrc.yaml"));
        candidates.push(PathBuf::from(&home).join(".csrc.yaml"));
    }

    for candidate in &candidates {
        if candidate.exists() {
            return (candidate.clone(), true);
        }
    }

    // Fallback to first candidate, or current dir if no candidates.
    let fallback = candidates
        .first()
        .cloned()
        .unwrap_or_else(|| PathBuf::from("."));
    (fallback, false)
}
