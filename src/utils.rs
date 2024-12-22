use regex::Regex;
use std::env;
use std::path::Path;

pub fn is_valid_executable_env_path(command: &str) -> Option<String> {
    let env_var_path = env::var("PATH").unwrap();

    for p in env_var_path.split(":") {
        let path_string = format!("{p}/{command}").to_string();
        let path = Path::new(&path_string);

        if path.is_file() {
            return Some(path_string);
        }
    }

    None
}

pub fn process_args(args_string: &String) -> Vec<String> {
    let regex_single_quotes_pattern: Regex = Regex::new(
        r#"'([^']*)'|"([^"]*)"|((?:\.\./|\./|/)?(?:[\w-]+/)*[\w-]+)|((?:\.\./|\./|\.)+)|(\w+)|(/)"#,
    )
    .unwrap();

    let captures: regex::CaptureMatches<'_, '_> =
        regex_single_quotes_pattern.captures_iter(&args_string);

    let mut results: Vec<String> = Vec::new();

    for capture in captures {
        match capture.get(1) {
            Some(c) => results.push(c.as_str().to_string()),
            None => match capture.get(2) {
                Some(c) => results.push(c.as_str().to_string()),
                None => match capture.get(3) {
                    Some(c) => results.push(c.as_str().to_string()),
                    None => match capture.get(4) {
                        Some(c) => results.push(c.as_str().to_string()),
                        None => match capture.get(5) {
                            Some(c) => results.push(c.as_str().to_string()),
                            None => match capture.get(6) {
                                Some(c) => results.push(c.as_str().to_string()),
                                None => (),
                            },
                        },
                    },
                },
            },
        }
    }

    if results.is_empty() {
        results.push(args_string.to_string())
    }

    // println!("{:#?}", results);

    results
}
