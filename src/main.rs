use clap::ArgMatches;

mod cli;
mod calender_endpoint;

fn main() {
    let matches = cli::cli().get_matches(); // register the command

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            println!(
                "Following value for 'name' of 'add' was provided: {}",
                sub_matches.get_one::<String>("name").expect("required")
            );
            add_event(sub_matches);
        },
        Some(("show", _sub_matches)) => {
            println!(
                "You want to show your calendar? DENIED"
            );
            show_event(_sub_matches);
        }
        _ => unreachable!(),
    }
}

fn add_event(args: &ArgMatches) {

}

fn show_event(args: &ArgMatches) {

}