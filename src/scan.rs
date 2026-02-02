use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn get_source_directories(source_root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut repos = Vec::new();
    visit_dir(source_root, &mut repos)?;
    Ok(repos)
}

fn visit_dir(dir: &Path, repos: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if is_source_dir(dir) {
        repos.push(dir.to_path_buf());
        return Ok(());
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if !path.is_dir() || path.is_symlink() {
            continue;
        }

        visit_dir(&path, repos)?;
    }

    Ok(())
}

fn is_source_dir(path: &Path) -> bool {
    path.join(".git").is_dir()
}
