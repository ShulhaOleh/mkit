use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const REPO: &str = "ShulhaOleh/mkit";

fn sha256(path: &Path) -> Option<String> {
    Command::new("sha256sum")
        .arg(path)
        .output()
        .ok()
        .and_then(|o| {
            String::from_utf8_lossy(&o.stdout)
                .split_whitespace()
                .next()
                .map(str::to_string)
        })
}

pub fn run() -> Result<(), String> {
    let arch = Command::new("uname")
        .arg("-m")
        .output()
        .map_err(|e| format!("failed to detect arch: {e}"))?;

    let arch = String::from_utf8_lossy(&arch.stdout).trim().to_string();

    let binary = match arch.as_str() {
        "x86_64" => "mkit-x86_64-linux",
        other    => return Err(format!("unsupported architecture: {other}")),
    };

    let url = format!("https://github.com/{REPO}/releases/download/latest/{binary}");

    let current = env::current_exe()
        .map_err(|e| format!("failed to locate current binary: {e}"))?;

    let tmp = Path::new("/tmp/mkit-update");

    println!("downloading latest mkit...");

    let status = Command::new("curl")
        .args(["-fsSL", &url, "-o"])
        .arg(tmp)
        .status()
        .map_err(|e| format!("failed to run curl: {e}"))?;

    if !status.success() {
        return Err("download failed".to_string());
    }

    if sha256(&current) == sha256(tmp) {
        fs::remove_file(tmp).ok();
        println!("already up to date");
        return Ok(());
    }

    Command::new("chmod").args(["+x"]).arg(tmp).status().ok();

    fs::remove_file(&current)
        .map_err(|e| format!("failed to remove old binary (try running with sudo): {e}"))?;

    fs::copy(tmp, &current)
        .map_err(|e| format!("failed to write new binary: {e}"))?;

    fs::remove_file(tmp).ok();

    println!("mkit updated");
    Ok(())
}
