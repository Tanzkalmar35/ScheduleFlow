use icalendar::{Calendar, Event, Todo};

#[derive(Debug)]
pub struct ICalendarUtil {
    pub(crate) calendar: Calendar
}

/// A wrapper for icalendar::Calendar actions.
impl ICalendarUtil {

    /// Initializes a new icalendar::Calendar.
    pub fn init() -> ICalendarUtil {
        Self {
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
