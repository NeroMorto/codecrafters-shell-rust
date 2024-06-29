#[allow(unused_imports)]
use std::io::{self, Write};

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
            0 => {},
            _ => {
                let command_with_possible_args = input.trim();
                let (command, command_args) = command_with_possible_args.trim().split_once(" ").unwrap_or((command_with_possible_args, ""));
                match command {
                    "echo" => stdout.write_all(format!("{command_args}\r\n").as_bytes()).unwrap(),
                    "exit" => if command_args == "0" {
                        break;
                    },
                    _ => stdout.write_all(format!("{command}: command not found\r\n").as_bytes()).unwrap()
                }
                input.clear();
                print!("$ ");
                io::stdout().flush().unwrap();
            }

        }
    }
}
