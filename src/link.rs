use std::fs;
use std::io::{self, Write};
use std::os::unix::fs as unix_fs;
use std::path::Path;
use crate::modules::Module;

fn prompt_conflict(dest: &Path) -> bool {
    loop {
        print!(
            "conflict: {} already exists\n  [s] skip  [o] overwrite: ",
            dest.display()
        );
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();

        match input.trim() {
            "s" => return false,
            "o" => return true,
            _   => println!("please enter 's' or 'o'"),
        }
    }
}

pub fn configs(modules: &[Module], home_dir: &Path) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for module in modules {
        println!("  [{}]", module.name);
        for src in &module.config_files {
            let Some(file_name) = src.file_name() else { continue };
            let dest = home_dir.join(file_name);

            if dest.exists()
                && !dest.symlink_metadata().map(|m| m.file_type().is_symlink()).unwrap_or(false)
                && !prompt_conflict(&dest)
            {
                println!("skipped {}", dest.display());
                continue;
            }

            if dest.symlink_metadata().is_ok() {
                let _ = fs::remove_file(&dest);
            }

            match unix_fs::symlink(src, &dest) {
                Ok(_)  => println!("linked {} -> {}", dest.display(), src.display()),
                Err(e) => errors.push(format!("{}: {e}", dest.display())),
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}
