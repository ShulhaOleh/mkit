use std::fs;
use std::path::{Path, PathBuf};

pub struct Module {
    pub name: String,
    pub packages: Vec<String>,
    pub config_files: Vec<PathBuf>,
}

pub fn scan(modules_dir: &Path) -> Result<Vec<Module>, std::io::Error> {
    let mut modules = Vec::new();

    for entry in fs::read_dir(modules_dir)? {
        let entry = entry?;
        let path  = entry.path();
        if !path.is_dir() { continue; }

        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut packages     = Vec::new();
        let mut config_files = Vec::new();

        for file in fs::read_dir(&path)? {
            let file      = file?;
            let file_path = file.path();
            let file_name = file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            if file_name == "packages.dnf" {
                packages = read_packages(&file_path)?;
            } else if !file_name.starts_with("packages.") {
                config_files.push(file_path);
            }
        }

        modules.push(Module { name, packages, config_files });
    }

    Ok(modules)
}

fn read_packages(path: &Path) -> Result<Vec<String>, std::io::Error> {
    Ok(fs::read_to_string(path)?
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect())
}
