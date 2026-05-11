use std::path::{Path, PathBuf};
use std::process::Command;
use crate::{modules, install, setup, link};

pub fn clone_repo(url: &str, dest: &PathBuf) -> Result<(), String> {
    let status = Command::new("git")
        .args(["clone", "--", url])
        .arg(dest)
        .status()
        .map_err(|e| format!("failed to run git: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("git clone failed with {status}"))
    }
}

pub fn run(dotfiles_path: &Path, home_path: &Path) -> Result<(), String> {
    let modules_path = dotfiles_path.join("modules");

    let modules = match modules::scan(&modules_path) {
        Ok(m) if m.is_empty() => {
            return Err(format!("no modules found in {}", modules_path.display()));
        }
        Ok(m)  => m,
        Err(e) => return Err(format!("error scanning modules: {e}")),
    };

    println!("found {} module(s)", modules.len());

    install::packages(&modules)?;
    setup::run(&modules)?;

    if let Err(errors) = link::configs(&modules, home_path) {
        for e in &errors { eprintln!("warning: {e}"); }
        return Err("linking failed".to_string());
    }

    println!("done.");
    Ok(())
}
