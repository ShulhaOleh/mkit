use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn home_path() -> PathBuf {
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

pub fn is_root() -> bool {
    Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
}
