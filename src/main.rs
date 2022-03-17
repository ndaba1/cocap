use cmder::{Designation, Event, Formatter, Program};
use figlet_rs::FIGfont;

fn main() {
    let mut program = Program::new();

    program
        .version("0.1.0")
        .bin_name("cocap")
        .author("Victor Ndaba <vndabam@gmail.com>")
        .description("A wrapper around git to increase developer productivity");

    program
        .add_cmd()
        .command("stats [app-name]")
        .alias("status")
        .describe("Print out statistics about your latest commits")
        .action(|vals, opts| {
            dbg!(vals);
            dbg!(opts);
        })
        .build(&mut program);

    program
        .add_cmd()
        .command("add <dir-name>")
        .alias("track")
        .describe("Start tracking a new directory with git and cocap")
        .action(|vals, opts| {
            dbg!(vals);
            dbg!(opts);
        })
        .build(&mut program);

    program
        .add_cmd()
        .command("activate <dir-name>")
        .alias("act")
        .describe("Change directory into the project with the given name")
        .action(|vals, opts| {
            dbg!(vals);
            dbg!(opts);
        })
        .build(&mut program);

    program
        .add_cmd()
        .command("ui")
        .alias("serve")
        .describe("Start the UI server on a default or provided port")
        .option("-p --port", "Specify a custom port to start the server on")
        .action(|vals, opts| {
            dbg!(vals);
            dbg!(opts);
        })
        .build(&mut program);

    program.on(Event::OutputVersion, |p, v| {
        let standard_font = FIGfont::standand().unwrap();
        let figure = standard_font.convert("cocap").unwrap();

        let mut fmtr = Formatter::default();
        fmtr.add(Designation::Keyword, &figure.to_string());
        fmtr.add(Designation::Headline, &format!("cocap v{}\n", &v));
        fmtr.add(Designation::Description, &format!("{}\n", p.about));

        fmtr.print();
    });

    program.parse();
}
