#![allow(unused)]
use std::{env::current_dir, fs, os, path::PathBuf};

use cmder::ParserMatches;

use crate::hooks::{self, CocapEntry};

pub fn add(matches: ParserMatches) {
    let name = matches.get_arg("<dir-name>").unwrap();
    let current_dir = current_dir().expect("msg");

    // TODO: Check whether to incorporate git functionality
    let entry = if name.as_str() == "." {
        CocapEntry::new(current_dir)
    } else {
        // try find the dir
        let p_buf = PathBuf::from(name);
        let name = p_buf.file_name().unwrap();
        let path = current_dir.join(&name);
        if !path.exists() {
            eprintln!("The provided dir name does not exist");
            return;
        }

        CocapEntry::new(path)
    };

    // TODO: Handle recursive option
    if matches.contains_option("-r") {
        let depth = matches.get_option_arg("<depth>").unwrap();

        let root_dir = entry.full_path;
        let mut current_dir = root_dir.clone();
        let mut start = 0;

        while start.to_string() != depth {}
    } else {
        let e = entry.clone();
        if let Some(_) = hooks::find_entry(e.dir_name) {
            eprintln!("Entry already tracked with cocap")
        } else {
            hooks::add_entry(entry);
        }
    }
}
