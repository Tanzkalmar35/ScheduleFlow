use anyhow::Result;
use clap::ArgMatches;
use icalendar::{Calendar, Component, Event as calendar_event};

/// Creates a new calendar object.
#[allow(unused)]
pub fn create_calendar(name: &str, description: &str) -> Calendar {
    Calendar::new()
        .name(name)
        .description(description)
        .done()
}

/// Used for creating new events.
pub fn create_event(cmd_arg: &ArgMatches) -> Result<calendar_event> {
    println!("{:?}", cmd_arg);
    let name = cmd_arg.get_one::<String>("name").expect("error processing name!");
    Ok(
        calendar_event::new()
            .summary(name)
            .done()
    )
}
