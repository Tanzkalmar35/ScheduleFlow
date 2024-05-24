use icalendar::{Calendar, Event, Todo};

use crate::pg_driver::PgDriver;
use crate::table_calendars::CalendarDAO;

#[derive(Debug)]
pub struct ICalendarUtil {
    is_new: bool,
    table_calendar: CalendarDAO,
    pub(crate) calendar: Calendar,
}

/// A wrapper for icalendar::Calendar actions.
impl ICalendarUtil {
    /// Initializes a new icalendar::Calendar.
    pub fn init(calendar: Calendar) -> ICalendarUtil {
        Self {
            is_new: true,
            table_calendar: CalendarDAO::new(),
            calendar,
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.calendar.push(event).done();
    }

    /// Adds an icalendar::Todo to the calendar
    pub fn add_todo(&mut self, todo: Todo) {
        self.calendar.push(todo).done();
    }

    /// Stores the icalendar::Calendar in the database.
    pub fn store(&mut self, driver: &mut PgDriver) {
        self.is_new = false;
    }
}
