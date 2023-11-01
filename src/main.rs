mod cli;
mod calender_endpoint;

fn main() {
    let matches = cli::cli().get_matches(); // register the command

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            println!(
                "Following value for 'name' of 'add' was provided: {}",
                sub_matches.get_one::<String>("name").expect("required")
            )
        }
        _ => unreachable!(),
    }
}
