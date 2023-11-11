mod calendar;

use icalendar::Calendar;
use crate::calendar::{create_event, open_calendar_tui};

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
                .expect("Error creating a new event; sub_matches = " + sub_matches);
        },
        Some(("show", sub_matches)) => {
            //open_calendar_tui(calendar)
            //    .expect("Error opening the calendar in tui interface; calendar = " + &calendar);
        },
        Some(("open", sub_matches)) => {
            open_calendar_tui(calendar)
                .expect("Error opening the calendar in tui interface; calendar = " + &calendar);
        }
        _ => unreachable!(),
    }
}
