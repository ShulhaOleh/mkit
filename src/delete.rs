use std::fs;
use std::path::PathBuf;

pub fn run(file: &str) -> Result<(), String> {
    let link = PathBuf::from(file);

    let target = link
        .read_link()
        .map_err(|_| format!("{file} is not a symlink tracked by mkit"))?;

    fs::remove_file(&link)
        .map_err(|e| format!("failed to remove symlink {file}: {e}"))?;

    fs::rename(&target, &link)
        .map_err(|e| format!("failed to move {} back: {e}", target.display()))?;

    println!("{} untracked, file restored to {}", target.display(), link.display());
    Ok(())
}
