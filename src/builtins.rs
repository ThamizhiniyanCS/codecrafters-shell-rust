use crate::utils;
use std::env;
use std::path::Path;
use std::process::exit;

pub fn _cd(args_string: Option<String>) {
    match args_string {
        Some(args) => {
            let path = Path::new(&args);

            match path.is_dir() {
                true => match path.is_absolute() {
                    true => env::set_current_dir(path).unwrap(),
                    false => env::set_current_dir(path.canonicalize().unwrap()).unwrap(),
                },
                false => println!("cd: {args}: No such file or directory"),
            }
        }
        None => println!("Expecting a valid path as argument."),
    }
}

pub fn _echo(args_string: Option<String>) {
    match args_string {
        Some(args) => println!("{}", args),
        None => println!(),
    }
}

pub fn _exit(args_string: Option<String>) {
    match args_string {
        Some(args) => exit(args.parse::<i32>().unwrap()),
        None => exit(0),
    }
}

pub fn _pwd() {
    println!("{}", env::current_dir().unwrap().to_str().unwrap());
}

pub fn _type(args_string: Option<String>) {
    let commands: [&str; 5] = ["cd", "echo", "exit", "pwd", "type"];

    match args_string {
        Some(arg) => match commands.contains(&arg.as_str()) {
            true => println!("{} is a shell builtin", arg),
            false => {
                let result: Option<String> = utils::is_valid_executable_env_path(&arg);

                match result {
                    Some(res) => println!("{} is {}", arg, res),
                    None => println!("{}: not found", arg),
                }
            }
        },
        None => println!("Expected a valid command as argument."),
    }
}
