use clap::ArgMatches;
use icalendar::{Event, Component, Calendar};

/// Creates a new calendar object.
pub fn create_calendar(name: &str, description: &str) -> Calendar {
    Calendar::new()
        .name(name)
        .description(description)
        .done()
}

/// Used for creating new events.
pub fn create_event(cmd_arg: &ArgMatches) -> Result<Event, Err()> {
    let name = cmd_arg.get_one::<String>("name").expect("error processing name!");
    Ok(
        Event::new()
        .summary(name)
        .done()
    )
}

/// Opens a tui window with the calendar.
pub fn open_calendar_tui(calendar: Calendar) -> Result<(), Err()> {
    Ok(())
}
