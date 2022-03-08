use clap::ArgMatches;
use cocap::{self, Program};

fn main() {
    let program = Program::new();

    let cfg = program.get_matches();

    dbg!(&cfg);

    match cfg.subcommand() {
        Some(cmd) => execute_cmd(cmd.0, cmd.1),
        None => {
            println!("Your command wasn't recognized")
        }
    }
}

fn execute_cmd(name: &str, vals: &ArgMatches) {
    match name {
        "stats" => cocap::cmd::stats(vals),
        _ => {}
    }
}
