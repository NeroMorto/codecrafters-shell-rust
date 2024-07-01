use crate::builtins::BuiltinCommand;
use crate::command::{Executable, ExternalCommand};
use std::str::FromStr;

pub fn parse_command(input: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let command = parts[0].to_string();
    let args = parts[1..].iter().map(|s| s.to_string()).collect();
    (command, args)
}


pub fn handle_command(command: String, args: &[String]) -> Result<String, String> {
    if let Ok(builtin) = BuiltinCommand::from_str(&command) {
        builtin.execute(&args)
    } else {
        let external_command = ExternalCommand {command};
        external_command.execute(&args)
    }
}
