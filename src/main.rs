mod modules;
mod install;
mod link;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: mkit <repo-url>");
        std::process::exit(1);
    }

    let url  = &args[1];
    let home = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("error: HOME not set");
        std::process::exit(1);
    });

    let home_path     = PathBuf::from(&home);
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
        for e in &errors {
            eprintln!("warning: {e}");
        }
        std::process::exit(1);
    }

    println!("done.");
}
