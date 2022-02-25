use clap::{arg, App};

pub struct Program;

impl Program {
    pub fn new() -> App<'static> {
        App::new("cocap")
            .about("Your very own coding co-captain.")
            .version("0.1.0")
            .author("Victor Ndaba <vndabam@gmail.com>")
            .subcommand(
                App::new("stats")
                    .about("Outputs the global stats of your environment")
                    .arg(arg!([project] "Display stats about a specific project"))
                    .alias("statistics"),
            )
            .subcommand(
                App::new("add")
                    .about("Start tracking the passed app")
                    .arg(arg!(-p --project <project> "The name of the directory/project to start tracking"))
                    .alias("track"),
            )
            .subcommand(
                App::new("remove")
                    .about("Stop tracking the passed app")
                    .arg(arg!(-p --project <project> "The name of the directory/project to stop tracking"))
                    .alias("forget"),
            )
            .subcommand(
                App::new("push")
                    .about("Push changes to respective origins")
                    .args(&[arg!(-p --project <project> "Push changes for a single project"),
                    arg!(-a --all "Push all the changes from the tracked projects")])
                    .alias("sync"),
            )
            .subcommand(
                App::new("ui")
                    .about("Starts the cocap UI server")
                    .args(&[arg!(-p --port <port> "Start the UI server on a specific port"),
                    arg!(-o --open "Open the server address in a browser window")])
                    .alias("visual"),
            )
    }
}
