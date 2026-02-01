use std::{env, path::PathBuf};

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
