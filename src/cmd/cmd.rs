use clap::{Arg, Command};

/// Creates the command line commands.
pub fn cmd() -> Command {
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
                        .short('t')
                        .help("The time the event ends")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("user")
                .arg_required_else_help(true)
                .about("Creates a new user")
                .arg(
                    Arg::new("firstname")
                        .long("firstname")
                        .short('f')
                        .help("The first name of the user")
                        .required(true)
                )
        )
}
