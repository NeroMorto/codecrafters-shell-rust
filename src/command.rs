use std::process::Command;

pub trait Executable {
    fn execute(&self, args: &[String]) -> Result<String, String>;
}

pub struct ExternalCommand {
    pub command: String,
}

impl Executable for ExternalCommand {
    fn execute(&self, args: &[String]) -> Result<String, String> {
        match self.search_in_path_env() {
            Ok(_) => {
                let mut cmd = Command::new(&self.command);
                cmd.args(args);
                let output = cmd.output().map_err(|e| e.to_string())?;
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            }
            Err(error) => Ok(error)
        }

    }
}

impl ExternalCommand {
    fn search_in_path_env(&self) -> Result<(), String> {
        let path_env = std::env::var("PATH").unwrap();
        let paths = path_env.split(":");
        for path_dir in paths {
            let command_path = format!("{path_dir}/{command}", command = &self.command);
            if std::path::Path::new(&command_path).exists() {
                return Ok(());
            }
        }
        Err(format!("{command}: command not found", command = &self.command))
    }
}