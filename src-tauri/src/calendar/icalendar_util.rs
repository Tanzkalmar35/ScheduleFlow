use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::error::Error;
use std::ops::DerefMut;

use serde::{Deserialize, Serialize};

use crate::db::db_actions::DbActions;
use crate::db::model::calendar::Calendar;
use crate::db::model::user::User;
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
    pub fn init() -> Self {
        Self {
            is_new: true,
            calendar: Calendar::new(),
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
    pub fn get_user_calendars(user: &User) -> Vec<Calendar> {
        let res: Vec<Calendar> = vec![];
        let user_uuid_matches = format!("user_uuid = '{}'", user.get_uuid());
        let matching_user_calendar_combinations = UserCalendarCombinationRepository::retrieve(
            driver().lock().unwrap().deref_mut(),
            Some(user_uuid_matches),
        );

        //for combination in matching_user_calendar_combinations {
        //    let calendar_uuid_matches = format!("");
        //    CalendarRepository::retrieve(
        //        driver().lock().unwrap().deref_mut(),
        //        Some(calendar_uuid_matches),
        //    )
        //}
        todo!()
    }
}

#[tauri::command]
pub fn get_calendar_of_current_user() -> Vec<Calendar> {
    ICalendarUtil::get_user_calendars(get_current_user().lock().unwrap().as_ref().unwrap())
}
