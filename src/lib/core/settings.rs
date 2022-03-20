use std::env::vars;
use std::path::Path;

pub fn check_config() {
    let full_path = if cfg!(windows) {
        let prefix = get_env_value("HOMEDRIVE").unwrap();
        let path = get_env_value("HOMEPATH").unwrap();

        Path::new(&prefix).join(path)
    } else {
        let path = get_env_value("HOME").unwrap();

        Path::new(&path).to_path_buf()
    };

    dbg!(full_path);
}

fn get_env_value(val: &str) -> Option<String> {
    let vals: Vec<_> = vars().collect();
    let result = vals.iter().find(|v| v.0 == val);

    match result {
        None => None,
        Some(v) => Some(v.1.clone()),
    }
}
