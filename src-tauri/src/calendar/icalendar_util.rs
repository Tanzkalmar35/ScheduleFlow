use icalendar::{Calendar, Component, Event, EventLike, Todo};
use crate::db_actions::DbActions;

use crate::pg_driver::PgDriver;
use crate::table_calendars::ICalendar;
use crate::table_components;
use crate::table_components::ComponentType;

#[derive(Debug)]
pub struct ICalendarUtil {
    is_new: bool,
    table_calendar: ICalendar,
    pub(crate) calendar: Calendar,
}

/// A wrapper for icalendar::Calendar actions.
impl ICalendarUtil {
    /// Initializes a new icalendar::Calendar.
    pub fn init(table_calendar: ICalendar) -> ICalendarUtil {
        Self {
            is_new: false,
            table_calendar,
            calendar: Calendar::new().done(),
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

    pub fn store(&mut self, driver: &mut PgDriver) {
        self.table_calendar.store(driver);
        if self.is_new {
            for component in self.calendar.components {
                if let Some(c_type) = component.as_event() {
                    table_components::IComponent::new(ComponentType::parse("event"));
                    self.calendar.push(Event::new().starts())
                }
            }
        }
    }
}
