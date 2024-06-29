#[allow(unused_imports)]
use std::io::{self, Write};
use std::time::Duration;

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
            0 => {
                std::thread::sleep(Duration::from_secs(1))
            },
            _ => {
                if input.trim() == "exit 0" {
                    break;
                }

                stdout.write_all(format!("{command}: command not found\r\n", command = input.trim()).as_bytes()).unwrap();
                input.clear();
                print!("$ ");
                io::stdout().flush().unwrap();
            }
        }
    }
}
