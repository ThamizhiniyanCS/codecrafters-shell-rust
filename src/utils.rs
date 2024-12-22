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
