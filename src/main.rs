use std::io::{self, Write};

use builtins::BuiltinCommand;

mod builtins;
mod utils;
mod command;

// enum CommandType {
//     Builtin(BuiltinCommand),
//     External,
// }
//
// impl FromStr for CommandType {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match BuiltinCommand::from_str(s) {
//             Ok(builtin) => Ok(CommandType::Builtin(builtin)),
//             Err(_) => Ok(CommandType::External)
//         }
//     }
// }

fn search_in_path_env(command: &str) -> Result<String, ()> {
    let path_env = std::env::var("PATH").unwrap();
    let paths = path_env.split(":");
    for path_dir in paths {
        let command_path = format!("{path_dir}/{command}");
        if std::path::Path::new(&command_path).exists() {
            return Ok(command_path.to_string());
        }
    }
    Err(())
}


// fn handle_builtin_command(builtin_command: BuiltinCommand, command_args: &str) -> Result<String, ()> {
//     match builtin_command {
//         BuiltinCommand::Echo => builtins::echo_command(command_args),
//         BuiltinCommand::Exit => builtins::exit_command(command_args),
//         BuiltinCommand::Type => type_command(command_args),
//         BuiltinCommand::Pwd => builtins::pwd_command(command_args),
//         BuiltinCommand::ChangeDirectory => builtins::change_directory_command(command_args)
//     }
// }

fn command_line() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

pub fn type_command(args: &[String]) -> Result<String, String> {
    let command = args.get(0).ok_or("No command provided".to_string())?;
    match BuiltinCommand::from_str(command) {
        Some(c) => Ok(BuiltinCommand::describe_command(c)),
        None => {
            match search_in_path_env(&command) {
                Ok(command_path) => Ok(format!("{command} is {command_path}")),
                Err(_) => Ok(format!("{command}: not found"))
            }
        }
    }
}


fn main() {
    let mut input = String::new();
    let stdin = io::stdin();



    loop {
        command_line();
        stdin.read_line(&mut input).unwrap();

        // Wait for user input
        if input.trim().is_empty() {
            continue;
        }

        let (command, args) = utils::parse_command(&input);
        match utils::handle_command(command, &args) {
            Ok(output) => {
                if !output.is_empty() {
                    println!("{}", output.trim())
                }
            }
            Err(error) => println!("Error: {}", error)
        }

        input.clear();
    }
}

