use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::error::Error;
use std::ops::DerefMut;

use serde::{Deserialize, Serialize};

use crate::db::db_actions::DbActions;
use crate::db::model::calendar::Calendar;
use crate::db::model::simple::simple_calendar::SimpleCalendar;
use crate::db::model::user::User;
use crate::db::model::user_calendar_combination::UserCalendarCombination;
use crate::db::pg_driver::PgDriver;
use crate::db::repository::calendar_repository::CalendarRepository;
use crate::db::repository::user_calendar_combination_repository::UserCalendarCombinationRepository;
use crate::runtime_objects::{driver, get_current_user};

#[derive(Debug)]
pub(crate) struct ICalendarUtil {
    is_new: bool,
    pub(crate) calendar: Calendar,
}

/// A wrapper for icalendar::Calendar actions.
impl ICalendarUtil {
    /// Initializes a new ICalendarUtil from an existing calendar.
    pub fn from(calendar: Calendar) -> ICalendarUtil {
        Self {
            is_new: false,
            calendar,
        }
    }

    /// Initializes a new ICalendarUtil.
    pub fn init(calendar_name: String) -> Self {
        Self {
            is_new: true,
            calendar: Calendar::new(calendar_name),
        }
    }

    /// Adds an icalendar::Event to the calendar.
    //pub fn add_event(&mut self, event: Event) {
    //    self.calendar.push(event).done();
    //}

    /// Adds an icalendar::Todo to the calendar.
    //pub fn add_todo(&mut self, todo: Todo) {
    //    self.calendar.push(todo).done();
    //}

    /// Stores the icalendar::Calendar in the database.
    //pub fn store(&mut self, driver: &mut PgDriver) {
    //    self.is_new = false;
    //}

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
    pub fn get_user_calendars(user: &User) -> Vec<SimpleCalendar> {
        let res: Vec<Calendar> = vec![];
        let mut simple_user_calendars: Vec<SimpleCalendar> = vec![];
        let mut driver_binding = driver().lock().unwrap();

        let calendars = UserCalendarCombinationRepository::get_calendars_of_user(
            driver_binding.deref_mut(),
            user,
        );

        for calendar in calendars {
            simple_user_calendars.push(SimpleCalendar::build(driver_binding.deref_mut(), calendar));
        }

        simple_user_calendars
    }
}

#[tauri::command]
pub fn get_calendar_of_current_user() -> Vec<SimpleCalendar> {
    ICalendarUtil::get_user_calendars(get_current_user().lock().unwrap().as_ref().unwrap())
}
