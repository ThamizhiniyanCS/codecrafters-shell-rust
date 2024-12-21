use regex::Regex;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn command_not_found(command: &str) {
    println!("{}: command not found", command);
}

fn main() {
    let commands: [&str; 3] = ["echo", "type", "exit"];

    let regex_echo_pattern: Regex = Regex::new(r"^echo\s(.+)$").unwrap();
    let regex_type_pattern: Regex = Regex::new(r"^type\s(.+)$").unwrap();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        input = input.trim().to_string();

        let echo_capture: Option<regex::Captures<'_>> = regex_echo_pattern.captures(&input);
        let type_capture: Option<regex::Captures<'_>> = regex_type_pattern.captures(&input);

        if input == "exit 0" {
            exit(0);
        } else if !echo_capture.is_none() {
            println!("{}", echo_capture.unwrap().get(1).unwrap().as_str());
        } else if !type_capture.is_none() {
            let command: &str = type_capture.unwrap().get(1).unwrap().as_str();

            if commands.contains(&command) {
                println!("{} is a shell builtin", command);
            } else {
                command_not_found(command);
            }
        } else {
            command_not_found(&input);
        }
    }
}
