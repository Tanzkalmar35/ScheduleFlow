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
                        .short('t')
                        .help("The tiem the event ends")
                        .required(true)
                )
                .arg(
                    Arg::new("color")
                        .long("color")
                        .short('c')
                        .help("The color of the event")
                )
        )
        .subcommand(
            Command::new("show")
                .about("Shows the calendar of a specified date period")
                .arg(
                    Arg::new("date")
                        .long("date")
                        .short('d')
                        .help("The date that will be used")
                        .required_unless_present_all(["from", "to"])
                )
                .arg(
                    Arg::new("from")
                        .long("from")
                        .short('f')
                        .help("The starting date of the printed calendar")
                        .required_unless_present("date")
                )
                .arg(
                    Arg::new("to")
                        .long("to")
                        .short('t')
                        .help("The ending date of the printed calendar")
                        .required_unless_present("date")
                )
        )
}
