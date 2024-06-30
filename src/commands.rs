use std::process::Command;

pub trait Executable {
    fn execute(self) -> Result<String, String>;
}

pub struct ExternalCommand {
    pub command: String,
    pub args: Vec<u8>
}

impl Executable for ExternalCommand {
    fn execute(self) -> Result<String, String> {
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args);
        let output = cmd.output().map_err(|error| error.to_string())?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}