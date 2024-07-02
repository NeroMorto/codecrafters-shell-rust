use std::env;
use std::process::exit;
use std::str::FromStr;

use crate::command::Executable;
use crate::type_command;

pub enum BuiltinCommand {
    Echo,
    Exit,
    Type,
    Pwd,
    ChangeDirectory,
}

impl Executable for BuiltinCommand {
    fn execute(&self, args: &[String]) -> Result<String, String> {
        match self {
            BuiltinCommand::Echo => echo_command(args),
            BuiltinCommand::Exit => exit_command(args),
            BuiltinCommand::Type => type_command(args),
            BuiltinCommand::Pwd => pwd_command(args),
            BuiltinCommand::ChangeDirectory => change_directory_command(args),
        }
    }
}


impl BuiltinCommand {
    pub fn describe_command(command: BuiltinCommand) -> String {
        match command {
            BuiltinCommand::Echo => "echo is a shell builtin".to_string(),
            BuiltinCommand::Exit => "exit is a shell builtin".to_string(),
            BuiltinCommand::Type => "type is a shell builtin".to_string(),
            BuiltinCommand::Pwd => "pwd is a shell builtin".to_string(),
            BuiltinCommand::ChangeDirectory => "cd is a shell builtin".to_string(),
        }
    }
}

impl FromStr for BuiltinCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(BuiltinCommand::Echo),
            "exit" => Ok(BuiltinCommand::Exit),
            "type" => Ok(BuiltinCommand::Type),
            "pwd" => Ok(BuiltinCommand::Pwd),
            "cd" => Ok(BuiltinCommand::ChangeDirectory),
            _ => Err(())
        }
    }
}
pub fn change_directory_command(args: &[String]) -> Result<String, String> {
    let path_to_change = args.get(0).map_or("~", |s| s.as_str());
    if path_to_change == "~" {
        env::set_current_dir(env::var("HOME").map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    } else {
        // More simple way, but is not satisfy challenge tests
        // env::set_current_dir(path_to_change).map_err(|e| e.to_string())?;
        let possible_directory = std::path::Path::new(path_to_change);
        match possible_directory.is_dir() && possible_directory.exists() {
            true => {
                env::set_current_dir(possible_directory).map_err(|e| e.to_string())?;
            }
            false => {
                return Ok(format!("cd: {path}: No such file or directory", path = possible_directory.display()))
            }
        }
    }
    Ok("".to_string())
}

pub fn pwd_command(_args: &[String]) -> Result<String, String> {
    let current_dir = std::env::current_dir().map_err(|e|e.to_string())?;
    Ok(current_dir.display().to_string())
}

pub fn echo_command(args: &[String]) -> Result<String, String> {
    Ok(args.join(" "))
}

pub fn exit_command(args: &[String]) -> Result<String, String> {
    let exit_code = args.get(0).ok_or("Exit code not provided".to_string())?;
    let exit_code = exit_code.parse::<i32>().map_err(|s| s.to_string())?;
    if exit_code == 0 {
        exit(exit_code)
    }
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo() {
        let result = echo_command(["Hello,".into(), "world!".into()].as_ref()).unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_pwd() {
        let result = pwd_command([].as_ref()).unwrap();
        assert_eq!(result, format!("{}", env::current_dir().unwrap().display().to_string()));
    }

    #[test]
    fn test_type_external_command() {
        let result = type_command(["ls".into()].as_ref()).unwrap();
        assert_eq!(result, "ls is /usr/bin/ls");
    }

    #[test]
    fn test_type_internal_command() {
        let result = type_command(["echo".into()].as_ref()).unwrap();
        assert_eq!(result, "echo is a shell builtin");
    }
}