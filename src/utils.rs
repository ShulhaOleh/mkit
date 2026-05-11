use std::env;
use std::path::PathBuf;

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
