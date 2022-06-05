use cmder::{Designation, Event, Formatter, Program, Setting};
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
        .subcommand("stats")
        .alias("status")
        .description("Print out statistics about your latest commits")
        .option("-d --dir <name>", "Show stats for a specific directory")
        .option("--count <number>", "The number of stats to show")
        .action(cmd::stats);

    program
        .subcommand("add")
        .argument("<dir-name>", "The directory to add")
        .alias("track")
        .description("Start tracking a new directory with git and cocap")
        .option(
            "-r --recursive <depth>",
            "Recursively add all subdirectories at provided depth",
        )
        .action(cmd::add);

    program
        .subcommand("activate")
        .argument("<dir-name>", "The directory to activate")
        .alias("act")
        .description("Change directory into the project with the given name")
        .action(cmd::activate);

    program
        .subcommand("ui")
        .alias("server")
        .description("Start the UI server on a default or provided port")
        .option(
            "-p --port <port-number>",
            "Specify a custom port to start the server on",
        )
        .action(|m| {
            dbg!(m);
        });

    program
        .subcommand("push")
        .alias("p")
        .description("Push changes to their respective remote repositories")
        .option("-a --all", "Push all changes in all tracked directories")
        .action(|m| {
            dbg!(m);
        });

    program
        .subcommand("config")
        .alias("cfg")
        .description("Set the configurations for cocap")
        .option("-g --global", "Configure global cocap options")
        .action(|m| {
            dbg!(m);
        });

    program
        .subcommand("remove")
        .alias("rm")
        .argument("<dir-name>", "The name of the directory/project")
        .description("Stop tracking the given directory name")
        .action(cmd::remove);

    program
        .subcommand("clean")
        .alias("reset")
        .description("Remove all tracked projects and reset to default config")
        .action(cmd::clean);

    program.on(Event::OutputVersion, |cfg| {
        let standard_font = FIGfont::standand().unwrap();
        let figure = standard_font.convert("cocap").unwrap();
        let prog = cfg.get_program();

        let mut fmtr = Formatter::new(prog.get_theme().to_owned());
        fmtr.add(Designation::Keyword, &figure.to_string());
        fmtr.add(
            Designation::Headline,
            &format!("cocap v{}\n", prog.get_version()),
        );
        fmtr.add(
            Designation::Description,
            &format!("{}\n", prog.get_description()),
        );

        fmtr.print();
    });

    program.set(Setting::HideCommandAliases(false));

    program.parse();
}
