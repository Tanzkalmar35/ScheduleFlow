use clap::ArgMatches;
mod calendar;

#[path = "util/cli_util.rs"]
mod cli_util;

fn main() {
    let matches = cli_util::cli().get_matches();

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
