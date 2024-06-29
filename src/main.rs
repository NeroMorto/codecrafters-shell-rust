#[allow(unused_imports)]
use std::io::{self, Write};
use std::str::FromStr;

enum CommandInfo {
    Echo,
    Exit,
    Type,
}

impl CommandInfo {
    pub fn describe_command(command: CommandInfo) -> String {
        match command {
            CommandInfo::Echo => "echo is a shell builtin".to_string(),
            CommandInfo::Exit => "exit is a shell builtin".to_string(),
            CommandInfo::Type => "type is a shell builtin".to_string(),
        }
    }
}

impl FromStr for CommandInfo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(CommandInfo::Echo),
            "exit" => Ok(CommandInfo::Exit),
            "type" => Ok(CommandInfo::Type),
            nonexistent => Err(format!("{}: not found", nonexistent))
        }
    }
}

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();


    let mut stdout = io::stdout();


    loop {
        stdin.read_line(&mut input).unwrap();
        match input.len() {
            0 => {}
            _ => {
                let command_with_possible_args = input.trim();
                let (command, command_args) = command_with_possible_args.trim().split_once(" ").unwrap_or((command_with_possible_args, ""));
                match CommandInfo::from_str(command) {
                    Ok(command) => {
                        match command {
                            CommandInfo::Echo => stdout.write_all(format!("{command_args}\r\n").as_bytes()).unwrap(),
                            CommandInfo::Exit => if command_args == "0" {
                                break;
                            }
                            CommandInfo::Type => {
                                match CommandInfo::from_str(command_args) {
                                    Ok(c) => stdout.write_all(format!("{info}\r\n", info = CommandInfo::describe_command(c)).as_bytes()).unwrap(),
                                    Err(err) => stdout.write_all(format!("{err}\r\n").as_bytes()).unwrap()
                                };
                            }
                        }
                    }

                    _ => stdout.write_all(format!("{command}: command not found\r\n").as_bytes()).unwrap()
                }
                input.clear();
                print!("$ ");
                io::stdout().flush().unwrap();
            }
        }
    }
}
