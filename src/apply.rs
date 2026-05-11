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

pub fn run(dotfiles_path: &Path, home_path: &Path) {
    let modules_path = dotfiles_path.join("modules");

    let modules = match modules::scan(&modules_path) {
        Ok(m) if m.is_empty() => {
            eprintln!("no modules found in {}", modules_path.display());
            std::process::exit(1);
        }
        Ok(m)  => m,
        Err(e) => {
            eprintln!("error scanning modules: {e}");
            std::process::exit(1);
        }
    };

    println!("found {} module(s)", modules.len());

    if let Err(e) = install::packages(&modules) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }

    if let Err(e) = setup::run(&modules) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }

    if let Err(errors) = link::configs(&modules, home_path) {
        for e in &errors { eprintln!("warning: {e}"); }
        std::process::exit(1);
    }

    println!("done.");
}
