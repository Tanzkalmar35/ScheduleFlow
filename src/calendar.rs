use clap::ArgMatches;
use icalendar::{Event, Component};

pub fn add_event(cmd_arg: &ArgMatches) -> Event {
    let name = cmd_arg.get_one::<String>("name").expect("error processing name!");
    Event::new()
        .summary(name)
        .done()
}