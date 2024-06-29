#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let mut stdout = io::stdout();

    match input.len() {
        0 => {},
        _ => {
            stdout.write_all(format!("{command}: command not found\r\n", command = input.trim()).as_bytes()).unwrap();
        }
    }
}
