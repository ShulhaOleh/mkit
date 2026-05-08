mod modules;
mod install;
mod link;
mod add;
mod delete;
mod update;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn clone_repo(url: &str, dest: &PathBuf) -> Result<(), String> {
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

fn apply(dotfiles_path: &Path, home_path: &Path) {
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

    if let Err(errors) = link::configs(&modules, home_path) {
        for e in &errors { eprintln!("warning: {e}"); }
        std::process::exit(1);
    }

    println!("done.");
}

fn home_path() -> PathBuf {
    // when running under sudo, use the real user's home instead of /root
    if let Ok(sudo_user) = env::var("SUDO_USER") {
        let path = PathBuf::from("/home").join(sudo_user);
        if path.exists() {
            return path;
        }
    }
    let home = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("error: HOME not set");
        std::process::exit(1);
    });
    PathBuf::from(home)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("update") => {
            if let Err(e) = update::run() {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }

        Some("add") => {
            if args.len() < 4 {
                eprintln!("usage: mkit add <file> <module>");
                std::process::exit(1);
            }
            let dotfiles_path = home_path().join("dotfiles");
            if let Err(e) = add::run(&args[2], &args[3], &dotfiles_path) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }

        Some("delete") => {
            if args.len() < 3 {
                eprintln!("usage: mkit delete <file>");
                std::process::exit(1);
            }
            if let Err(e) = delete::run(&args[2]) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }

        Some(url) => {
            let home_path     = home_path();
            let dotfiles_path = home_path.join("dotfiles");

            if dotfiles_path.exists() {
                println!("dotfiles already at {}, skipping clone", dotfiles_path.display());
            } else {
                println!("cloning {} -> {}", url, dotfiles_path.display());
                if let Err(e) = clone_repo(url, &dotfiles_path) {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            }

            apply(&dotfiles_path, &home_path);
        }

        None => {
            eprintln!("usage: mkit <repo-url>");
            eprintln!("       mkit add <file> <module>");
            eprintln!("       mkit delete <file>");
            eprintln!("       mkit update");
            std::process::exit(1);
        }
    }
}
