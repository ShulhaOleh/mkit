use std::process::Command;
use crate::modules::Module;

fn is_root() -> bool {
    Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
}

pub fn packages(modules: &[Module]) -> Result<(), String> {
    let packages: Vec<&str> = modules
        .iter()
        .flat_map(|m| m.packages.iter().map(String::as_str))
        .collect();

    if packages.is_empty() {
        println!("no packages to install");
        return Ok(());
    }

    println!("installing {} package(s)...", packages.len());

    for m in modules {
        if !m.packages.is_empty() {
            println!("  [{}] {}", m.name, m.packages.join(" "));
        }
    }

    let status = if is_root() {
        Command::new("dnf")
            .args(["install", "-y"])
            .args(&packages)
            .status()
            .map_err(|e| format!("failed to run dnf: {e}"))?
    } else {
        Command::new("sudo")
            .args(["dnf", "install", "-y"])
            .args(&packages)
            .status()
            .map_err(|e| format!("failed to run sudo dnf: {e}"))?
    };

    if status.success() {
        Ok(())
    } else {
        Err(format!("dnf exited with {status}"))
    }
}
