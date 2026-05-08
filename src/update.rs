use std::path::Path;
use std::process::Command;

pub fn run(dotfiles_path: &Path) -> Result<(), String> {
    if !dotfiles_path.exists() {
        return Err(format!(
            "{} not found — run mkit <repo-url> first",
            dotfiles_path.display()
        ));
    }

    let status = Command::new("git")
        .args(["-C", dotfiles_path.to_str().unwrap_or_default(), "pull"])
        .status()
        .map_err(|e| format!("failed to run git: {e}"))?;

    if !status.success() {
        return Err(format!("git pull failed with {status}"));
    }

    Ok(())
}
