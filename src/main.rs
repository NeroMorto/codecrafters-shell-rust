use std::io::{self, Write};
use std::str::FromStr;

use builtins::BuiltinCommand;

mod builtins;
mod utils;
mod command;

fn search_in_path_env(command: &str) -> Result<String, String> {
    let path_env = std::env::var("PATH").map_err(|e| e.to_string())?;
    let paths = path_env.split(":");
    for path_dir in paths {
        let command_path = format!("{path_dir}/{command}");
        if std::path::Path::new(&command_path).exists() {
            return Ok(command_path.to_string());
        }
    }
    Err("".to_string())
}


fn command_line(){
    print!("$ ");
    match io::stdout().flush() {
        Ok(_) => {}
        Err(error) => println!("Error: {error}")
    }
}

pub fn type_command(args: &[String]) -> Result<String, String> {
    let command = args.get(0).ok_or("No command provided".to_string())?;
    match BuiltinCommand::from_str(command) {
        Ok(c) => Ok(BuiltinCommand::describe_command(c)),
        Err(_) => {
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
        match stdin.read_line(&mut input) {
            Ok(_) => {
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
            }
            Err(error) => println!("Error wile reading a command: {error}")
        }


        input.clear();
    }
}

