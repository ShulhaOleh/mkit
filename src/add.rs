use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};

pub fn run(file: &str, module: &str, dotfiles_path: &Path) -> Result<(), String> {
    let src = PathBuf::from(file)
        .canonicalize()
        .map_err(|e| format!("{file}: {e}"))?;

    let file_name = src
        .file_name()
        .ok_or_else(|| format!("invalid path: {}", src.display()))?;

    let module_dir = dotfiles_path.join("modules").join(module);
    let dest       = module_dir.join(file_name);

    fs::create_dir_all(&module_dir)
        .map_err(|e| format!("failed to create {}: {e}", module_dir.display()))?;

    fs::rename(&src, &dest)
        .map_err(|e| format!("failed to move {}: {e}", src.display()))?;

    unix_fs::symlink(&dest, &src)
        .map_err(|e| format!("failed to symlink {}: {e}", src.display()))?;

    println!("{} -> {}", src.display(), dest.display());
    Ok(())
}
