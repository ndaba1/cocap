use std::process::Command;

use cmder::ParserMatches;

use crate::hooks::find_entry;

pub fn activate(matches: ParserMatches) {
    let ctx = matches.get_arg("<dir-name>").unwrap();

    // check if entry exists in entrydb
    if let Some(entry) = find_entry(ctx) {
        let path = entry.full_path;
        // let path = Path::from(path);

        let target = if cfg!(windows) { "windows" } else { "unix" };
        let shell = if target == "windows" { "cmd" } else { "sh" };
        let first_arg = if target == "windows" { "/C" } else { "-c" };

        Command::new(shell)
            .arg(first_arg)
            .args(["cd", path.as_str()])
            .status()
            .expect("Failed to switch context");
    } else {
        panic!("No such entry tracked with cocap")
    }
}
