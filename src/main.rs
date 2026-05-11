mod modules;
mod install;
mod setup;
mod link;
mod add;
mod delete;
mod update;
mod apply;
mod utils;

use std::env;
use utils::home_path;

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
            let home      = home_path();
            let dotfiles  = home.join("dotfiles");

            if dotfiles.exists() {
                println!("dotfiles already at {}, skipping clone", dotfiles.display());
            } else {
                println!("cloning {} -> {}", url, dotfiles.display());
                if let Err(e) = apply::clone_repo(url, &dotfiles) {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            }

            if let Err(e) = apply::run(&dotfiles, &home) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
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
