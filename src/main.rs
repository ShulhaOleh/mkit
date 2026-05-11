mod modules;
mod install;
mod setup;
mod link;
mod add;
mod delete;
mod update;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Cmd {
    name:  &'static str,
    usage: &'static str,
    run:   fn(&[String]) -> Result<(), String>,
}

const COMMANDS: &[Cmd] = &[
    Cmd {
        name:  "update",
        usage: "mkit update",
        run:   |_| update::run(),
    },
    Cmd {
        name:  "add",
        usage: "mkit add <file> <module>",
        run:   |args| {
            if args.len() < 2 {
                return Err("usage: mkit add <file> <module>".to_string());
            }
            add::run(&args[0], &args[1], &home_path().join("dotfiles"))
        },
    },
    Cmd {
        name:  "delete",
        usage: "mkit delete <file>",
        run:   |args| {
            if args.is_empty() {
                return Err("usage: mkit delete <file>".to_string());
            }
            delete::run(&args[0])
        },
    },
];

fn is_repo_url(s: &str) -> bool {
    s.contains("://") || s.starts_with("git@") || s.starts_with('/') || s.starts_with("./")
}

fn print_usage() {
    eprintln!("usage:");
    eprintln!("  mkit <repo-url>");
    for cmd in COMMANDS {
        eprintln!("  {}", cmd.usage);
    }
}

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

fn home_path() -> PathBuf {
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
        Some(name) if COMMANDS.iter().any(|c| c.name == name) => {
            let cmd = COMMANDS.iter().find(|c| c.name == name).unwrap();
            if let Err(e) = (cmd.run)(&args[2..]) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }

        Some(url) if is_repo_url(url) => {
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

        Some(unknown) => {
            eprintln!("unknown command: {unknown}");
            print_usage();
            std::process::exit(1);
        }

        None => {
            print_usage();
            std::process::exit(1);
        }
    }
}
