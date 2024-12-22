use regex::Regex;
use std::io::{self, Write};
use std::process::Command;

mod builtins;
mod utils;

fn execute(cmd: &str, args_string: Option<String>) {
    let result: Option<String> = utils::is_valid_executable_env_path(&cmd);

    if result.is_none() {
        println!("{}: command not found", cmd);
    } else {
        let output = match args_string {
            Some(args) => Command::new(cmd)
                .args(args.split_whitespace())
                .output()
                .expect("Failed to execute process."),
            None => Command::new(cmd)
                .output()
                .expect("Failed to execute process."),
        };

        let stdout = String::from_utf8(output.stdout).expect("Invalid utf-8 in process output");

        println!("{}", stdout.trim());
    }
}

fn main() {
    let regex_command_pattern: Regex = Regex::new(r"^(\w+)(?:\s(.+))?$").unwrap();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let trimmed_input = input.trim().to_string();

        let command_captures: Option<regex::Captures<'_>> =
            regex_command_pattern.captures(&trimmed_input);

        let mut command: Option<&str> = Some("");
        let mut args_string: Option<String> = Some(String::new());

        if !command_captures.is_none() {
            let capture = command_captures.unwrap();

            command = if capture.get(1).is_none() {
                None
            } else {
                Some(capture.get(1).unwrap().as_str())
            };

            args_string = if capture.get(2).is_none() {
                None
            } else {
                Some(capture.get(2).unwrap().as_str().to_string())
            };
        }

        if !command.is_none() {
            match command.unwrap() {
                "cd" => builtins::_cd(args_string),
                "echo" => builtins::_echo(args_string),
                "exit" => builtins::_exit(args_string),
                "pwd" => builtins::_pwd(),
                "type" => builtins::_type(args_string),
                _ => execute(command.unwrap(), args_string),
            }
        }
    }
}
