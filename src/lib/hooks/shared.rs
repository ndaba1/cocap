use std::path::PathBuf;

use directories::UserDirs;

pub fn get_home_dir() -> PathBuf {
    if let Some(dir) = UserDirs::new() {
        dir.home_dir().to_path_buf()
    } else {
        panic!("Could not locate valid home dir")
    }
}
