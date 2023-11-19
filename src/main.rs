use icalendar::Calendar;

use crate::calendar::{create_event, open_calendar_tui};

mod calendar;

#[path = "util/command_util.rs"]
mod command_util;
mod tui;

fn main() {
    // We need to store the calendar in a file and get it out of it instead of
    // creating a new one every time the application starts.
    let calendar = Calendar::new();

    let matches = command_util::cmd().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            create_event(sub_matches)
                .expect("Error creating a new event");
        }
        Some(("show", _sub_matches)) => {
            //open_calendar_tui(calendar)
            //    .expect("Error opening the calendar in tui interface; calendar = " + &calendar);
        }
        Some(("open", _sub_matches)) => {
            open_calendar_tui(calendar)
                .expect("Error opening the calendar in tui interface");
        }
        _ => unimplemented!(),
    }
}
