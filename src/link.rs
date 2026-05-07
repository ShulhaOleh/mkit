use std::os::unix::fs as unix_fs;
use std::path::Path;
use crate::modules::Module;

pub fn configs(modules: &[Module], home_dir: &Path) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for module in modules {
        println!("  [{}]", module.name);
        for src in &module.config_files {
            let Some(file_name) = src.file_name() else { continue };
            let dest = home_dir.join(file_name);

            if dest.symlink_metadata().is_ok() {
                let _ = std::fs::remove_file(&dest);
            }

            match unix_fs::symlink(src, &dest) {
                Ok(_)  => println!("linked {} -> {}", dest.display(), src.display()),
                Err(e) => errors.push(format!("{}: {e}", dest.display())),
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
