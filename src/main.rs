use clap::ArgMatches;
use icalendar::Calendar;

use crate::calendar::{create_event, open_calendar_tui};
use crate::config::User;

#[path = "util/command_util.rs"]
mod command_util;

mod calendar;
mod config;
mod tui;
mod db;
mod schema;

#[tokio::main]
async fn main() {
    // We need to store the calendar in a file and get it out of it instead of
    // creating a new one every time the application starts.

    let calendar = Calendar::new();
    let user = User::new(&ArgMatches::default());

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
            open_calendar_tui(calendar, &user)
                .expect("Error opening the calendar in tui interface");
        }
        Some(("config", sub_matches)) => {
            let user = User::new(sub_matches);
            println!("Successfully created new user {:?}", user);
        }
        _ => unimplemented!("This command is not implemented yet"),
    }
}
