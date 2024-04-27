use icalendar::{Calendar, Event, Todo};
use crate::table_calendars::ICalendar;

#[derive(Debug)]
pub struct ICalendarUtil {
    table_calendar: ICalendar,
    pub(crate) calendar: Calendar
}

/// A wrapper for icalendar::Calendar actions.
impl ICalendarUtil {

    /// Initializes a new icalendar::Calendar.
    pub fn init(table_calendar: ICalendar) -> ICalendarUtil {
        Self {
            table_calendar,
            calendar: Calendar::new().done()
        }
    }

    /// Adds an icalendar::Event to the calendar.
    pub fn add_event(&mut self, event: Event) {
        self.calendar.push(event).done();
    }

    /// Adds an icalendar::Todo to the calendar
    pub fn add_todo(&mut self, todo: Todo) {
        self.calendar.push(todo).done();
    }
}
