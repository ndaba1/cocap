use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::utils::CocapResult;

use super::shared::get_home_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CocapConfig {
    avg_commits: i32,
    use_color: bool,
}

impl Default for CocapConfig {
    fn default() -> Self {
        Self {
            avg_commits: 5,
            use_color: true,
        }
    }
}

/**
 * >>> loads the file at $HOME/User/.cocap/cocap.toml
 * >>> the cocap config should have an index of all saved dirs, but what if dir name changes?
 * perhaps each addition is a struct with
 * {
 *     project_base_name: string
 *     hash: string
 *     project_location: PathBuf
 * }
 */

pub fn get_config() -> CocapResult<CocapConfig> {
    // TODO: Check for local configs
    let home = get_home_dir();
    let cocap_path = home.join(Path::new(".cocap"));

    if cocap_path.exists() {
        // load the cocap cfg files
        let contents = fs::read_to_string(cocap_path.join("config.json"))?;
        let cfg: CocapConfig = serde_json::from_str(&contents)?;

        Ok(cfg)
    } else {
        // load default cocap options and create cocap dir
        let refs_path = cocap_path.join(Path::new("refs"));

        fs::create_dir(cocap_path.clone())?;
        fs::create_dir(refs_path.clone())?;

        let entries = serde_json::json!({
            "entries": []
        });
        let entries = serde_json::to_string_pretty(&entries)?;
        let mut entries_file = File::create(refs_path.join("entries.json"))?;
        entries_file.write_all(entries.as_bytes())?;

        let default_cfg = CocapConfig::default();
        let mut cfg_file = File::create(cocap_path.join("config.json"))?;

        let contents = serde_json::to_string_pretty(&default_cfg)?;
        cfg_file.write_all(contents.as_bytes())?;

        Ok(default_cfg)
    }
}
