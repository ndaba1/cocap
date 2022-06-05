#![allow(unused)]
use std::{
    io::{self, Write},
    path::Path,
    process::Command,
};

use cmder::ParserMatches;
use git2::Repository;
use tabled::{Table, Tabled};

use crate::hooks;

#[derive(Tabled)]
pub struct CocapStat {
    name: &'static str,
    total_commits: i32,
    unpushed_commits: i32,
    today_s_commits: i32,
}

pub fn stats(_matches: ParserMatches) {
    let _cfg = hooks::get_config();
    let mut stats_vec = vec![];

    // load entries
    let db = hooks::load_entries().unwrap();

    // iterate over each entry getting stats
    let target = if cfg!(windows) { "windows" } else { "unix" };
    let shell = if target == "windows" { "cmd" } else { "sh" };
    let first_arg = if target == "windows" { "/C" } else { "-c" };

    for e in db.entries {
        let dir = e.full_path;
        let dir = Path::new(&dir);

        // spawn a new process, to load git stats
        let mut cmd = Command::new(shell);

        cmd.arg(first_arg);
        cmd.current_dir(dir);
        cmd.args(["git", "log"]);

        let out = cmd.output().expect("some error ocuuredd");
        stats_vec.push(out);
    }

    // sort stats accordingly
    // for o in stats_vec {
    //     io::stdout().write_all(&o.stdout);
    // }
    let data = [
        CocapStat {
            name: "first",
            today_s_commits: 10,
            total_commits: 100,
            unpushed_commits: 20,
        },
        CocapStat {
            name: "second",
            today_s_commits: 4,
            total_commits: 10,
            unpushed_commits: 5,
        },
        CocapStat {
            name: "third",
            today_s_commits: 10,
            total_commits: 150,
            unpushed_commits: 20,
        },
        CocapStat {
            name: "fourth",
            today_s_commits: 20,
            total_commits: 106,
            unpushed_commits: 20,
        },
    ];

    let table = Table::new(data);
    let val = table.to_string();
    println!("{val}");

    // present stats

    println!("Cocap stats command invoked")
}
