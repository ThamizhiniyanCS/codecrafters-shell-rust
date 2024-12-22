use crate::utils;
use std::env;
use std::path::Path;
use std::process::exit;

pub fn _cd(args_string: Option<Vec<String>>) {
    match args_string {
        Some(args) => match args[0] == "~" {
            true => {
                let home = env::var("HOME").unwrap();
                env::set_current_dir(home).unwrap();
            }
            false => {
                let path = Path::new(&args[0]);

                match path.is_dir() {
                    true => match path.is_absolute() {
                        true => env::set_current_dir(path).unwrap(),
                        false => env::set_current_dir(path.canonicalize().unwrap()).unwrap(),
                    },
                    false => println!("cd: {}: No such file or directory", args[0]),
                }
            }
        },
        None => println!("Expecting a valid path as argument."),
    }
}

pub fn _echo(args_string: Option<Vec<String>>) {
    match args_string {
        Some(args) => {
            for (i, arg) in args.iter().enumerate() {
                match i == 0 {
                    true => print!("{arg}"),
                    false => print!(" {arg}"),
                }
            }
            println!()
        }
        None => println!(),
    }
}

pub fn _exit(args_string: Option<Vec<String>>) {
    match args_string {
        Some(args) => exit(args[0].parse::<i32>().unwrap()),
        None => exit(0),
    }
}

pub fn _pwd() {
    println!("{}", env::current_dir().unwrap().to_str().unwrap());
}

pub fn _type(args_string: Option<Vec<String>>) {
    let commands: [&str; 5] = ["cd", "echo", "exit", "pwd", "type"];

    match args_string {
        Some(args) => {
            for arg in args {
                match commands.contains(&arg.as_str()) {
                    true => println!("{} is a shell builtin", arg),
                    false => {
                        let result: Option<String> = utils::is_valid_executable_env_path(&arg);

                        match result {
                            Some(res) => println!("{} is {}", arg, res),
                            None => println!("{}: not found", arg),
                        }
                    }
                }
            }
        }
        None => println!("Expected a valid command as argument."),
    }
}
