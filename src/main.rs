use regex::Regex;
use std::io::{self, Write};
use std::sync::LazyLock;

mod builtins;
mod utils;

pub static REGEX_COMMAND_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^((?:.*?/)?'.+'|".+"|\w+)(?:\s(.+))?$"#).unwrap());
pub static IS_MULTIPLE_ARGUMENTS_REGEX_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?:(?:\s["'])|(?:["']\s))"#).unwrap());
pub static ESCAPE_BACKSLASH_INSIDE_DOUBLE_QUOTES_REGEX_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\\\$|\\"|\\\\n|\\\\|")"#).unwrap());
pub static ENCLOSED_SINGLE_QUOTES_REGEX_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"('[^']*')"#).unwrap());
pub static ENCLOSED_DOUBLE_QUOTES_REGEX_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(".*")"#).unwrap());

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let trimmed_input = input.trim().to_string();

        let command_captures: Option<regex::Captures<'_>> =
            REGEX_COMMAND_PATTERN.captures(&trimmed_input);

        let mut command: Option<&str> = Some("");
        let mut args_string: Option<String> = Some(String::new());

        if command_captures.is_none() {
            continue;
        } else {
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

        let processed_args: Option<Vec<(usize, String)>> = match args_string {
            Some(args_str) => Some(utils::parse_args(&args_str)),
            None => None,
        };

        if !command.is_none() {
            match command.unwrap() {
                "cd" => builtins::_cd(processed_args),
                "echo" => builtins::_echo(processed_args),
                "exit" => builtins::_exit(processed_args),
                "pwd" => builtins::_pwd(),
                "type" => builtins::_type(processed_args),
                _ => utils::execute(command.unwrap(), processed_args),
            }
        }
    }
}
