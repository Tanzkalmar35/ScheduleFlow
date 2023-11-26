use anyhow::Result;
use clap::ArgMatches;
use icalendar::{Calendar, Component, Event as calendar_event};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::config::User;
use crate::tui::event::{Event, EventHandler};
use crate::tui::Tui;
use crate::tui::tui_app::TuiApp;

#[path = "tui/action.rs"]
mod action;

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
    let name = cmd_arg.get_one::<String>("name").expect("error processing name!");
    Ok(
        calendar_event::new()
            .summary(name)
            .done()
    )
}

/// Opens a tui window with the calendar.
pub fn open_calendar_tui(_calendar: Calendar, user: &User) -> Result<()> {
    let mut app = TuiApp::new();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(user)?;
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => {
                action::update(&mut app, key_event);
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
