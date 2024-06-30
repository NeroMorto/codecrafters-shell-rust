use std::env;
use std::str::FromStr;

pub enum BuiltinCommand {
    Echo,
    Exit,
    Type,
    Pwd,
    ChangeDirectory,
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(BuiltinCommand::Echo),
            "exit" => Ok(BuiltinCommand::Exit),
            "type" => Ok(BuiltinCommand::Type),
            "pwd" => Ok(BuiltinCommand::Pwd),
            "cd" => Ok(BuiltinCommand::ChangeDirectory),
            _ => Err(s.to_string())
        }
    }
}
pub fn change_directory_command(args: &str) -> Result<String, ()> {
    if args == "~" {
        env::set_current_dir(env::var("HOME").unwrap()).unwrap();
        return Ok("".to_string());
    }

    let possible_directory = std::path::Path::new(args);
    Ok(match possible_directory.is_dir() && possible_directory.exists() {
        true => {
            env::set_current_dir(args).unwrap();
            "".to_string()
        }
        false => {
            format!("cd: {args}: No such file or directory")
        }
    })
}

pub fn pwd_command(_args: &str) -> Result<String, ()> {
    let current_dir = std::env::current_dir().unwrap();
    Ok(current_dir.display().to_string())
}

pub fn echo_command(args: &str) -> Result<String, ()> {
    Ok(args.to_string())
}

pub fn exit_command(args: &str) -> Result<String, ()> {
    if args == "0" {
        return Err(());
    }
    Ok("".to_string())
}

