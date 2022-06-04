use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::{get_config, shared::get_home_dir};
use crate::utils::CocapResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CocapEntry {
    pub dir_name: String,
    pub full_path: String,
}

impl CocapEntry {
    pub fn new(path: PathBuf) -> Self {
        // FIXME: Remove unwraps, better err handling
        let name = path.file_name().unwrap();
        let p_str = path.to_str().unwrap();
        let name = name.to_str().unwrap();

        Self {
            dir_name: name.into(),
            full_path: p_str.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryDB {
    entries: Vec<CocapEntry>,
}

fn get_entries_path() -> CocapResult<PathBuf> {
    // assert cfg files exist
    get_config()?;

    Ok(get_home_dir()
        .join(".cocap")
        .join("refs")
        .join("entries.json"))
}

pub fn add_entry(entry: CocapEntry) -> CocapResult<()> {
    // load entries
    let path = get_entries_path()?;
    let mut values = load_entries()?;

    // add new entry
    values.entries.push(entry);
    let contents = serde_json::to_string_pretty(&values)?;
    fs::write(path, contents)?;

    Ok(())
}

pub fn load_entries() -> CocapResult<EntryDB> {
    let path = get_entries_path()?;
    let contents = fs::read_to_string(path)?;
    let values: EntryDB = serde_json::from_str(&contents)?;

    Ok(values)
}

pub fn find_entry(val: String) -> Option<CocapEntry> {
    let values = load_entries().unwrap();

    for v in values.entries {
        if v.dir_name == val || v.full_path == val {
            return Some(v);
        }
    }

    None
}
