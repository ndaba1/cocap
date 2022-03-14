use cmder::Program;

fn main() {
    let mut program = Program::new();

    program
        .version("0.1.0")
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

    program.parse();
}
