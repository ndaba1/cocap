use cmder::{Designation, Event, Formatter, Program};
use figlet_rs::FIGfont;

use cocap::cmd;

fn main() {
    let mut program = Program::new();

    program
        .version("0.1.0")
        .bin_name("cocap")
        .author("Victor Ndaba <vndabam@gmail.com>")
        .description("A wrapper around git to increase developer productivity");

    program
        .command("stats")
        .alias("status")
        .description("Print out statistics about your latest commits")
        .option(
            "-c --count <count-size>",
            "Specify how far the stats should show",
        )
        .option(
            "-d --dir <dir-name>",
            "Print out stats for a specific directory",
        )
        .action(cmd::stats)
        .build(&mut program);

    program
        .command("add <dir-name>")
        .alias("track")
        .description("Start tracking a new directory with git and cocap")
        .option(
            "-r --root",
            "Track all git initialized directories in a given root dir",
        )
        .action(cmd::add)
        .build(&mut program);

    program
        .command("activate <dir-name>")
        .alias("act")
        .description("Change directory into the project with the given name")
        .action(|vals, opts| {
            dbg!(vals);
            dbg!(opts);
        })
        .build(&mut program);

    program
        .command("ui")
        .alias("serve")
        .description("Start the UI server on a default or provided port")
        .option("-p --port", "Specify a custom port to start the server on")
        .action(|vals, opts| {
            dbg!(vals);
            dbg!(opts);
        })
        .build(&mut program);

    program
        .command("push [dir-name]")
        .alias("p")
        .description("Push changes to their respective remote repositories")
        .option("-a --all", "Push all changes in all tracked directories")
        .action(|_v, _o| {})
        .build(&mut program);

    program
        .command("config")
        .alias("cfg")
        .description("Set the configurations for cocap")
        .option("-g --global", "Configure global cocap options")
        .action(|_v, _o| {})
        .build(&mut program);

    program.on(Event::OutputVersion, |p, v| {
        let standard_font = FIGfont::standand().unwrap();
        let figure = standard_font.convert("cocap").unwrap();

        let mut fmtr = Formatter::default();
        fmtr.add(Designation::Keyword, &figure.to_string());
        fmtr.add(Designation::Headline, &format!("cocap v{}\n", &v));
        fmtr.add(
            Designation::Description,
            &format!("{}\n", p.get_description()),
        );

        fmtr.print();
    });

    program.parse();
}
