use std::env;
use std::process::Command;
use crate::modules::Module;

pub fn run(modules: &[Module]) -> Result<(), String> {
    for module in modules {
        let script = module.path.join("install.sh");
        if !script.exists() {
            continue;
        }

        println!("  [{}] running install.sh", module.name);

        let mut cmd = match env::var("SUDO_USER") {
            Ok(user) => {
                let mut c = Command::new("sudo");
                c.args(["-u", &user, "bash"]);
                c
            }
            Err(_) => Command::new("bash"),
        };

        let status = cmd
            .arg(&script)
            .status()
            .map_err(|e| format!("failed to run install.sh for {}: {e}", module.name))?;

        if !status.success() {
            return Err(format!("install.sh failed for module {}", module.name));
        }
    }

    Ok(())
}
