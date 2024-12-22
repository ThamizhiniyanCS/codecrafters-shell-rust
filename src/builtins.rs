use crate::utils;
use std::env;
use std::process::exit;

pub fn _echo(args_string: Option<String>) {
    if !args_string.is_none() {
        println!("{}", args_string.unwrap());
    }
}

pub fn _exit(args_string: Option<String>) {
    if args_string.is_none() {
        exit(0)
    } else {
        exit(args_string.unwrap().parse::<i32>().unwrap());
    }
}

pub fn _pwd() {
    println!("{}", env::current_dir().unwrap().to_str().unwrap());
}

pub fn _type(args_string: Option<String>) {
    let commands: [&str; 4] = ["echo", "type", "exit", "pwd"];

    if !args_string.is_none() {
        let arg = args_string.unwrap();

        if commands.contains(&arg.as_str()) {
            println!("{} is a shell builtin", arg);
        } else {
            let result: Option<String> = utils::is_valid_executable_env_path(&arg);

            if result.is_none() {
                println!("{}: not found", arg);
            } else {
                println!("{} is {}", arg, result.unwrap());
            }
        }
    }
}
