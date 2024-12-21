use regex::Regex;
use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

fn builtin_type(env_var_path: &String, command: &str) -> Option<String> {
    for p in env_var_path.split(":") {
        let path_string = format!("{p}/{command}").to_string();
        let path = Path::new(&path_string);

        if path.is_file() {
            println!("{command} is {path_string}");
            return Some(path_string);
        }
    }

    None
}

fn main() {
    let commands: [&str; 3] = ["echo", "type", "exit"];
    let env_var_path = env::var("PATH").unwrap();
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

            let result: Option<String> = builtin_type(&env_var_path, command);

            if result.is_none() {
                println!("{}: not found", command);
            } else {
                println!("{command} is {}", result.unwrap());
            }
        } else {
            println!("{}: command not found", input);
        }
    }
}
