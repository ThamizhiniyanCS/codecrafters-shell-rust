use regex::{Regex, RegexSet};
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

fn is_multiple_arguments(args_string: &String) -> bool {
    let regex_pattern: Regex = Regex::new(r#"(?:(?:\s")|(?:"\s))"#).unwrap();

    regex_pattern.is_match(&args_string)
}

fn is_separate_argument(
    m: regex::Match<'_>,
    characters: &Vec<char>,
    characters_len: usize,
) -> Option<String> {
    let before_0: char = match m.start() == 0 {
        true => ' ',
        false => characters[m.start() - 1],
    };
    let before_1: char = match m.start() == 0 || m.start() == 1 {
        true => ' ',
        false => characters[m.start() - 2],
    };
    let after: char = match m.end() == characters_len {
        true => ' ',
        false => characters[m.end()],
    };

    match before_0 == ' ' && after == ' ' && before_1 != '\\' {
        true => Some(m.as_str().to_string()),
        false => None,
    }
}

pub fn process_args(args_string: &String) -> Vec<String> {
    let regex_expressions = [
        // Capture strings within Single quotes
        r#"('[^']*')"#,
        // Capture strings within Double quotes
        match is_multiple_arguments(args_string) {
            true => r#"((?:".*?")+)"#,
            false => r#"(".*")"#,
        },
        // Capture normal strings
        r#"(\w+)"#,
        // Capture normal and path strings without quotes and with backslashes
        r#"(~?(?:(?:(?:\.\./|\./|/)?(?:\w+)(?:\.\./|\./|/))+)?(?:(?:\w+)?(?:\\.)+(?:\w+)?(?:\\.\w+)*/?))"#,
        // Capture file paths without backslashes
        r#"(~?(?:(?:\.\./|\./|/)?(?:[\w-]+)/?)+)+"#,
        // Capture ./ and ../
        r#"((?:\.\./|\./)+)"#,
        // Capture ., ~ and /
        r#"(\.|/|~)"#,
    ];
    let characters: Vec<char> = args_string.chars().collect();
    let characters_len: usize = characters.len();
    let mut results: Vec<String> = Vec::new();

    let regex_set = RegexSet::new(regex_expressions).unwrap();

    let regexes: Vec<_> = regex_set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();

    for index in regex_set.matches(&args_string).into_iter() {
        match index {
            // Capture strings within Single quotes
            0 => {
                for capture in regexes[0].captures_iter(&args_string) {
                    match is_separate_argument(capture.get(1).unwrap(), &characters, characters_len)
                    {
                        Some(res) => {
                            results.push(
                                res.strip_prefix('\'')
                                    .and_then(|s| s.strip_suffix('\''))
                                    .unwrap_or(&res)
                                    .to_string(),
                            );
                        }
                        None => (),
                    }
                }
            }
            // Capture strings within Double quotes
            1 => {
                for capture in regexes[1].captures_iter(&args_string) {
                    match is_separate_argument(capture.get(1).unwrap(), &characters, characters_len)
                    {
                        Some(res) => {
                            results.push(
                                res.strip_prefix('"')
                                    .and_then(|s| s.strip_suffix('"'))
                                    .unwrap_or(&res)
                                    .replace("\\\\", "\\")
                                    .replace("\\$", "$")
                                    .replace("\\\"", "\"")
                                    .replace("\\\n", "\n"),
                            );
                        }
                        None => (),
                    }
                }
            }
            // Capture normal strings
            2 => {
                for capture in regexes[2].captures_iter(&args_string) {
                    match is_separate_argument(capture.get(1).unwrap(), &characters, characters_len)
                    {
                        Some(res) => results.push(res),
                        None => (),
                    }
                }
            }
            // Capture normal and path strings without quotes and with backslashes
            3 => {
                for capture in regexes[3].captures_iter(&args_string) {
                    match is_separate_argument(capture.get(1).unwrap(), &characters, characters_len)
                    {
                        Some(res) => {
                            let mut temp = res;

                            if !temp.contains('/') {
                                temp = temp.replace('\\', "");
                            };

                            results.push(temp);
                        }
                        None => (),
                    }
                }
            }
            // Capture file paths without backslashes
            4 => {
                for capture in regexes[4].captures_iter(&args_string) {
                    match is_separate_argument(capture.get(1).unwrap(), &characters, characters_len)
                    {
                        Some(res) => {
                            if res.contains("/") {
                                results.push(res)
                            }
                        }
                        None => (),
                    }
                }
            }
            // Capture ./ and ../
            5 => {
                for capture in regexes[5].captures_iter(&args_string) {
                    match is_separate_argument(capture.get(1).unwrap(), &characters, characters_len)
                    {
                        Some(res) => results.push(res),
                        None => (),
                    }
                }
            }
            // Capture ., ~ and /
            6 => {
                for capture in regexes[6].captures_iter(&args_string) {
                    match is_separate_argument(capture.get(1).unwrap(), &characters, characters_len)
                    {
                        Some(res) => results.push(res),
                        None => (),
                    }
                }
            }
            _ => (),
        }
    }

    if results.is_empty() {
        results.push(args_string.to_string())
    }

    results
}
