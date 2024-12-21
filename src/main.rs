use regex::Regex;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let regex_echo_pattern: Regex = Regex::new(r"^echo\s(.+)$").unwrap();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        input = input.trim().to_string();

        let echo_capture = regex_echo_pattern.captures(&input);

        if input == "exit 0" {
            exit(0);
        } else if !echo_capture.is_none() {
            println!("{}", echo_capture.unwrap().get(1).unwrap().as_str());
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}
