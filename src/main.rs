use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::str::FromStr;

enum CommandType {
    Builtin(BuiltinCommand),
    External,
}

impl FromStr for CommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match BuiltinCommand::from_str(s) {
            Ok(builtin) => Ok(CommandType::Builtin(builtin)),
            Err(_) => Ok(CommandType::External)
        }
    }
}

enum BuiltinCommand {
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

fn search_in_path_env(command: &str) -> Result<String, ()> {
    let path_env = std::env::var("PATH").unwrap();
    let paths = path_env.split(":");
    for path_dir in paths {
        let command_path = format!("{path}/{command}", path = path_dir, command = command);
        if std::path::Path::new(&command_path).exists() {
            return Ok(command_path.to_string());
        }
    }
    Err(())
}

fn handle_type_command(possible_command: &str) -> String {
    match BuiltinCommand::from_str(possible_command) {
        Ok(c) => BuiltinCommand::describe_command(c),
        Err(command) => {
            match search_in_path_env(&command) {
                Ok(command_path) => format!("{command} is {command_path}"),
                Err(_) => format!("{command}: not found")
            }
        }
    }
}

fn handle_builtin_command(builtin_command: BuiltinCommand, command_args: &str) -> Result<String, ()> {
    match builtin_command {
        BuiltinCommand::Echo => Ok(command_args.to_string()),
        BuiltinCommand::Exit => {
            if command_args == "0" {
                return Err(());
            }
            Ok("".to_string())
        }
        BuiltinCommand::Type => {
            Ok(handle_type_command(command_args))
        }
        BuiltinCommand::Pwd => {
            let current_dir = std::env::current_dir().unwrap();
            Ok(current_dir.display().to_string())
        }
        BuiltinCommand::ChangeDirectory => {
            let possible_directory = std::path::Path::new(command_args);
            match possible_directory.is_dir() && possible_directory.exists() {
                true => {
                    env::set_current_dir(command_args).unwrap();
                    Ok("".to_string())
                }
                false => {
                    Ok(format!("cd: {command_args}: No such file or directory"))
                }
            }
        }
    }
}

fn command_line() {
    print!("$ ");
    io::stdout().flush().unwrap();
}


fn main() {
    let mut input = String::new();
    let stdin = io::stdin();

    command_line();

    loop {
        stdin.read_line(&mut input).unwrap();
        // Wait for user input
        match input.len() {
            0 => {}
            _ => {
                let command_with_possible_args = input.trim();
                let (command, command_args) = command_with_possible_args.trim().split_once(" ").unwrap_or((command_with_possible_args, ""));

                match CommandType::from_str(command) {
                    Ok(command_type) => match command_type {
                        CommandType::Builtin(command) => match handle_builtin_command(command, command_args) {
                            Ok(output) => {
                                if output.len() > 0 {
                                    println!("{output}");
                                }
                                io::stdout().flush().unwrap();
                            }
                            Err(_) => break
                        }
                        CommandType::External => match search_in_path_env(&command) {
                            Ok(command_path) => {
                                let mut cmd = Command::new(command_path);
                                cmd.stdout(Stdio::piped());
                                if command_args.len() > 0 {
                                    cmd.args(command_args.trim().split(" ")).spawn().unwrap();
                                }
                                let executable = cmd.spawn().unwrap();
                                let output = executable.wait_with_output().expect("Failed to execute a command");
                                print!("{}", String::from_utf8(output.stdout).unwrap())
                            }
                            Err(_) => println!("{command}: command not found")
                        }
                    }
                    Err(_) => {}
                }

                input.clear();
                command_line();
            }
        }
    }
}
