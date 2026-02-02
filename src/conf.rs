use std::{
    env,
    fmt::{self, Display},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use serde_yaml::Mapping as YamlMapping;
use serde_yaml::Value as YamlValue;

#[derive(Debug)]
pub enum ConfigError {
    ReadError(std::io::Error),
    ParseError(serde_yaml::Error),
    InvalidValue {
        field: String,
        value: String,
        details: String,
    },
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ReadError(e) => write!(f, "failed to read config file: {}", e),
            ConfigError::ParseError(e) => write!(f, "failed to parse config file: {}", e),
            ConfigError::InvalidValue {
                field,
                value,
                details,
            } => write!(
                f,
                "invalid value for field '{}': '{}' - {}",
                field, value, details
            ),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::ReadError(e) => Some(e),
            ConfigError::ParseError(e) => Some(e),
            ConfigError::InvalidValue { .. } => None,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "sourceRoot", default)]
    pub source_root: SourceRoot,

    #[serde(default)]
    pub cache: CacheConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceRoot(pub PathBuf);

impl Default for SourceRoot {
    fn default() -> Self {
        SourceRoot(PathBuf::from("~"))
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CacheConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub location: CacheLocation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheLocation(pub PathBuf);

impl Default for CacheLocation {
    fn default() -> Self {
        if let Some(cache_home) = env::var_os("XDG_CACHE_HOME") {
            return CacheLocation(PathBuf::from(cache_home).join("csrc"));
        }
        CacheLocation(PathBuf::from("~/.cache/csrc"))
    }
}

pub fn load_config(overrides: &[(String, String)]) -> Result<Config, ConfigError> {
    let mut merged = load_config_from_file()?;

    apply_overrides(&mut merged, load_config_from_env())?;

    let exlpicit_overrides = overrides.iter().map(|(key, val)| {
        let path: Vec<String> = key.split(".").map(|s| s.to_string()).collect();
        (path, val.clone())
    });

    apply_overrides(&mut merged, exlpicit_overrides)?;

    let config: Config =
        serde_yaml::from_value(YamlValue::Mapping(merged)).map_err(ConfigError::ParseError)?;

    Ok(config)
}

fn load_config_from_file() -> Result<YamlMapping, ConfigError> {
    match read_config_file() {
        None => Ok(YamlMapping::new()),
        Some(res) => match res {
            Err(e) => Err(ConfigError::ReadError(e)),
            Ok(contents) => {
                let yaml_value: YamlMapping =
                    serde_yaml::from_slice(&contents).map_err(ConfigError::ParseError)?;
                Ok(yaml_value)
            }
        },
    }
}

fn read_config_file() -> Option<Result<Vec<u8>, std::io::Error>> {
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
        match std::fs::read(candidate) {
            Ok(contents) => return Some(Ok(contents)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Some(Err(e)),
        }
    }

    None
}

fn load_config_from_env() -> Vec<(Vec<String>, String)> {
    env_to_config_overrides("CSRC__")
}

fn env_to_config_overrides(prefix: &str) -> Vec<(Vec<String>, String)> {
    std::env::vars()
        .filter(|(key, _)| key.starts_with(prefix))
        .map(|(key, val)| {
            let name = &key[prefix.len()..];
            let path: Vec<String> = name
                .split("__")
                .map(|s| screaming_snake_to_camel(s))
                .collect();
            (path, val)
        })
        .collect()
}

fn screaming_snake_to_camel(s: &str) -> String {
    let binding = s.to_lowercase();
    let words = binding.split('_');
    let mut iter = words.into_iter();
    let first = iter.next().unwrap_or("").to_string();
    let rest: String = iter
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(c) => c.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect();
    first + &rest
}

// TODO: Use override keys with at least one path segment so we don't need to check for empty.

fn apply_overrides<I>(map: &mut YamlMapping, overrides: I) -> Result<(), ConfigError>
where
    I: IntoIterator<Item = (Vec<String>, String)>,
{
    for (path, value) in overrides {
        let path_refs: Vec<&str> = path.iter().map(|s| s.as_str()).collect();
        let value = serde_yaml::from_str(&value).map_err(|e| ConfigError::InvalidValue {
            field: path.join("."),
            value: value.clone(),
            details: format!("failed to parse override value: {}", e),
        })?;
        apply_override(map, &path_refs, &value)?;
    }
    Ok(())
}

fn apply_override(
    map: &mut YamlMapping,
    path: &[&str],
    value: &YamlValue,
) -> Result<(), ConfigError> {
    // TODO: Catch conflicts like trying to override a non-mapping value with a mapping.

    if path.is_empty() {
        return Ok(());
    }

    if path.len() == 1 {
        map.insert(YamlValue::from(path[0]), value.clone());
        return Ok(());
    }

    let entry = map
        .entry(YamlValue::from(path[0]))
        .or_insert(YamlValue::Mapping(YamlMapping::new()));

    if let YamlValue::Mapping(m) = entry {
        return apply_override(m, &path[1..], value);
    }

    Ok(())
}
