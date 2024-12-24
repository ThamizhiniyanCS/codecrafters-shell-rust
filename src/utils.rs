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

pub fn process_args(args_string: &String) -> Vec<(usize, String)> {
    let is_multiple_arguments_regex_pattern: Regex =
        Regex::new(r#"(?:(?:\s["'])|(?:["']\s))"#).unwrap();
    let escape_backslash_inside_double_quotes_regex_pattern: Regex =
        Regex::new(r#"(\\\$|\\"|\\\\n|\\\\|")"#).unwrap();

    let regex_expressions = [
        // Capture strings within Single quotes
        r#"('[^']*')"#,
        // Capture strings within Double quotes
        match is_multiple_arguments_regex_pattern.is_match(&args_string) {
            true => r#"((?:".*?")+|(?:".*')+)"#,
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
    let mut results: Vec<(usize, String)> = Vec::new();

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
                            let temp = &res[1..res.len() - 1].to_string();

                            results.push((0, temp.clone()));
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
                            let mut temp: Vec<Option<char>> =
                                res.chars().map(|char| Some(char)).collect();

                            for capture in escape_backslash_inside_double_quotes_regex_pattern
                                .captures_iter(&res)
                            {
                                let m = capture.get(1).unwrap();
                                match m.as_str() {
                                    "\\\\" => {
                                        temp[m.start()] = Some('\\');
                                        temp[m.end() - 1] = None;
                                    }
                                    "\\$" => {
                                        temp[m.start()] = Some('$');
                                        temp[m.end() - 1] = None;
                                    }
                                    "\\\"" => {
                                        temp[m.start()] = Some('\"');
                                        temp[m.end() - 1] = None;
                                    }
                                    "\\\\n" => {
                                        temp[m.start()] = Some('\\');
                                        temp[m.start() + 1] = Some('n');
                                        temp[m.end() - 1] = None;
                                    }
                                    "\"" => {
                                        temp[m.start()] = None;
                                    }
                                    _ => (),
                                }
                            }

                            let mut final_string: String = String::new();

                            for opt in temp {
                                match opt {
                                    Some(c) => final_string.push(c),
                                    None => (),
                                }
                            }

                            results.push((1, final_string));
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
                        Some(res) => results.push((2, res)),
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

                            results.push((3, temp));
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
                                results.push((4, res))
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
                        Some(res) => results.push((5, res)),
                        None => (),
                    }
                }
            }
            // Capture ., ~ and /
            6 => {
                for capture in regexes[6].captures_iter(&args_string) {
                    match is_separate_argument(capture.get(1).unwrap(), &characters, characters_len)
                    {
                        Some(res) => results.push((6, res)),
                        None => (),
                    }
                }
            }
            _ => (),
        }
    }

    if results.is_empty() {
        results.push((010, args_string.to_string()))
    }

    results
}
