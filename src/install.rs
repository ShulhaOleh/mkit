use std::process::Command;
use crate::modules::Module;

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

    let status = Command::new("dnf")
        .arg("install")
        .arg("-y")
        .args(&packages)
        .status()
        .map_err(|e| format!("failed to run dnf: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("dnf exited with {status}"))
    }
}
