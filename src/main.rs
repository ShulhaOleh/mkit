mod modules;
mod install;
mod link;
mod add;

use std::env;
use std::path::PathBuf;
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

fn home_path() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("error: HOME not set");
        std::process::exit(1);
    });
    PathBuf::from(home)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
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

        Some(url) => {
            let home_path     = home_path();
            let dotfiles_path = home_path.join("dotfiles");
            let modules_path  = dotfiles_path.join("modules");

            if dotfiles_path.exists() {
                println!("dotfiles already at {}, skipping clone", dotfiles_path.display());
            } else {
                println!("cloning {} -> {}", url, dotfiles_path.display());
                if let Err(e) = clone_repo(url, &dotfiles_path) {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            }

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

            if let Err(errors) = link::configs(&modules, &home_path) {
                for e in &errors { eprintln!("warning: {e}"); }
                std::process::exit(1);
            }

            println!("done.");
        }

        None => {
            eprintln!("usage: mkit <repo-url>");
            eprintln!("       mkit add <file> <module>");
            std::process::exit(1);
        }
    }
}
