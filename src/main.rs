use cocap::program::Program;

fn main() {
    let program = Program::new();

    let cfg = program.get_matches();

    dbg!(cfg);
}
