use regex::Regex;
use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, Command};

fn is_valid_executable_env_path(env_var_path: &String, command: &str) -> Option<String> {
    for p in env_var_path.split(":") {
        let path_string = format!("{p}/{command}").to_string();
        let path = Path::new(&path_string);

        if path.is_file() {
            return Some(path_string);
        }
    }

    None
}

fn custom_echo(args_string: Option<String>) {
    if !args_string.is_none() {
        println!("{}", args_string.unwrap());
    }
}

fn custom_type(env_var_path: &String, args_string: Option<String>) {
    let commands: [&str; 3] = ["echo", "type", "exit"];

    if !args_string.is_none() {
        let arg = args_string.unwrap();

        if commands.contains(&arg.as_str()) {
            println!("{} is a shell builtin", arg);
        } else {
            let result: Option<String> = is_valid_executable_env_path(env_var_path, &arg);

            if result.is_none() {
                println!("{}: not found", arg);
            } else {
                println!("{} is {}", arg, result.unwrap());
            }
        }
    }
}

fn custom_exit(args_string: Option<String>) {
    if args_string.is_none() {
        exit(0)
    } else {
        exit(args_string.unwrap().parse::<i32>().unwrap());
    }
}

fn execute(env_var_path: &String, cmd: &str, args_string: Option<String>) {
    let result: Option<String> = is_valid_executable_env_path(env_var_path, &cmd);

    if result.is_none() {
        println!("{}: not found", cmd);
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
    let env_var_path = env::var("PATH").unwrap();
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
                "echo" => custom_echo(args_string),
                "type" => custom_type(&env_var_path, args_string),
                "exit" => custom_exit(args_string),
                _ => execute(&env_var_path, command.unwrap(), args_string),
            }
        }
    }
}
