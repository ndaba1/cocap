use std::io;

use cmder::ParserMatches;

use crate::hooks::{clear_entries, reset_config};

pub fn clean(_matches: ParserMatches) {
    // ask confirmation
    print!("You are about to reset cocap. Do you wish to continue? (y/N)");
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            let val = buf.replace("\n", "").replace("\r", "");
            match val.to_lowercase().as_str() {
                "y" => {
                    reset_config().expect("An error occurred");
                    clear_entries().expect("An error occurred");
                }
                _ => {
                    println!("Operation cancelled!");
                    std::process::exit(0);
                }
            }
        }
        Err(error) => println!("error: {}", error),
    }
}
