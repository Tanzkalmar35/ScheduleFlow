use std::borrow::Borrow;
use std::error::Error;
use std::ops::DerefMut;

use icalendar::{Calendar, Event, Todo};

use crate::db::db_actions::DbActions;
use crate::db::pg_driver::PgDriver;
use crate::db::tables::adapter::ICalendarAdapter;
use crate::db::tables::table_calendars::CalendarDAO;
use crate::db::tables::table_users::User;
use crate::runtime_objects::{driver, get_current_user};

#[derive(Debug)]
pub(crate) struct ICalendarUtil {
    is_new: bool,
    table_calendar: CalendarDAO,
    pub(crate) calendar: Calendar,
}

/// A wrapper for icalendar::Calendar actions.
impl ICalendarUtil {
    /// Initializes a new ICalendarUtil from an existing calendar.
    pub fn from(calendar: Calendar) -> ICalendarUtil {
        Self {
            is_new: false,
            table_calendar: CalendarDAO::new(),
            calendar,
        }
    }

    /// Initializes a new ICalendarUtil.
    pub fn init() -> Self {
        Self {
            is_new: true,
            table_calendar: CalendarDAO::new(),
            calendar: Calendar::new(),
        }
    }

    /// Adds an icalendar::Event to the calendar.
    pub fn add_event(&mut self, event: Event) {
        self.calendar.push(event).done();
    }

    /// Adds an icalendar::Todo to the calendar.
    pub fn add_todo(&mut self, todo: Todo) {
        self.calendar.push(todo).done();
    }

    /// Stores the icalendar::Calendar in the database.
    pub fn store(&mut self, driver: &mut PgDriver) {
        self.is_new = false;
    }

    /// Returns all calendars that are associated to the current user.
    ///
    /// # Examples
    ///
    /// ```
    /// let user: User = User::default();
    /// let calendars: Vec<Calendar> = ICalendarUtil::get_user_calendars(&user);
    /// for calendar in calendars {
    ///     println!("Calendar: {}", calendar);
    /// }
    /// ```
    pub fn get_user_calendars(user: &User) -> Vec<Calendar> {
        let mut calendars = vec![];
        let condition = format!("user_uuid = {}", user.uuid);

        let calendar_dao_list =
            CalendarDAO::retrieve(driver().lock().unwrap().deref_mut(), Some(condition));

        for calendar_dao in calendar_dao_list {
            calendars.push(ICalendarAdapter::bundle_calendar(
                driver().lock().unwrap().deref_mut(),
                calendar_dao,
            ));
        }

        calendars
    }
}

struct SerializableCalendar {}

#[tauri::command]
pub fn get_calendar_of_current_user() -> Result<Vec<Calendar>, ()> {
    let calendars =
        ICalendarUtil::get_user_calendars(get_current_user().lock().unwrap().as_ref().unwrap());
    Ok(calendars)
}
