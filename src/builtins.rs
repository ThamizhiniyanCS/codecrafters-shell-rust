use crate::utils;
use std::env;
use std::path::Path;
use std::process::exit;

pub fn _cd(args_string: Option<String>) {
    match args_string {
        Some(args) => {
            let path = Path::new(&args);

            if path.is_dir() {
                if path.is_absolute() {
                    env::set_current_dir(path).unwrap();
                }
            } else {
                println!("cd: {args}: No such file or directory");
            }
        }
        None => println!("Expecting a valid path as argument."),
    }
}

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
    let commands: [&str; 5] = ["cd", "echo", "exit", "pwd", "type"];

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
