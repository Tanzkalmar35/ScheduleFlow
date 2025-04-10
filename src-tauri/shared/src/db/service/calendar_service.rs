use crate::db::model::simple::simple_calendar::{self, SimpleCalendar};
use crate::db::model::user::User;
use crate::db::repository::user_calendar_combination_repository::UserCalendarCombinationRepository;
use crate::runtime_objects::{self, driver};
use std::ops::DerefMut;

pub struct CalendarService;

impl CalendarService {
    /// Returns all calendars that are associated to the current user.
    ///
    /// # Examples
    ///
    /// ```
    /// let user: User = User::default();
    /// let calendars: Vec<SimpleCalendar> = CalendarService::get_user_calendars(&user);
    /// for calendar in calendars {
    ///     println!("Calendar: {}", calendar);
    /// }
    /// ```
    pub fn get_user_calendars(user: &User) -> Vec<SimpleCalendar> {
        let mut simple_user_calendars: Vec<SimpleCalendar> = vec![];
        let mut driver_binding = driver().lock().unwrap();

        let calendars = UserCalendarCombinationRepository::get_calendars_of_user(
            driver_binding.deref_mut(),
            user,
        );

        for calendar in calendars {
            let simple_calendar = SimpleCalendar::build(driver_binding.deref_mut(), calendar);
            simple_user_calendars.push(simple_calendar);
            runtime_objects::cache_calendar(simple_calendar)
        }

        simple_user_calendars
    }
}
