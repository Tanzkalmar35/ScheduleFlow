use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("scheduleflow")
        .about("A cli calendar tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds a new calendar entry")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .short('n')
                        .help("The name of the event to be added")
                        .required(true)
                )
                .arg(
                    Arg::new("date")
                        .long("date")
                        .short('d')
                        .help("The date of the event")
                        .required(true)
                )
                .arg(
                    Arg::new("from")
                        .long("from")
                        .short('f')
                        .help("The time the event starts")
                        .required(true)
                )
                .arg(
                    Arg::new("to")
                        .long("to")
                        .help("The tiem the event ends")
                        .required(true)
                )
                .arg(
                    Arg::new("COLOR")
                        .long("color")
                        .short('c')
                        .help("The color of the event")
                )
        )
}
