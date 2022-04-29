#![allow(unused)]
use std::collections::HashMap;
use std::env::current_dir;

use super::super::core::settings;

pub fn add(vals: HashMap<String, String>, _opts: HashMap<String, String>) {
    // let config = settings::get_config();

    // let dir = vals.get("dir_name").unwrap();

    // let val = if dir == "." {
    //     current_dir().unwrap()
    // } else {
    //     let cleaned = dir.replace("./", "").replace(".\\", "");
    //     current_dir().unwrap().join(cleaned)
    // };
    // settings::push_ref(&config, val.to_str().unwrap());

    println!("OKAY");
}
