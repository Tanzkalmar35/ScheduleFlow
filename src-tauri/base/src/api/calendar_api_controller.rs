use std::ops::DerefMut;
use crate::api::auth_api_controller::user_exists;
use crate::db::db_actions::DbActions;
use crate::db::model::calendar::Calendar;
use crate::db::model::simple::simple_calendar::SimpleCalendar;
use crate::db::model::user_calendar_combination::UserCalendarCombination;
use crate::db::repository::calendar_repository::CalendarRepository;
use crate::db::repository::user_calendar_combination_repository::UserCalendarCombinationRepository;
use crate::db::repository::user_repository::UserRepository;
use crate::db::service::calendar_service::CalendarService;
use crate::runtime_objects::{driver, get_current_user};

#[tauri::command]
pub fn get_calendar_of_current_user() -> Vec<SimpleCalendar> {
    CalendarService::get_user_calendars(get_current_user().lock().unwrap().as_ref().unwrap())
}

#[tauri::command]
pub fn store_new_calendar(calendar_name: String, users_emails: Vec<String>) -> bool {
    let calendar = Calendar::new(calendar_name);
    let result = CalendarRepository::store(driver().lock().unwrap().deref_mut(), &calendar);
    // retrieve user uuids via emails
    for email in users_emails {
        let user = UserRepository::get_by_email(driver().lock().unwrap().deref_mut(), email).expect("Error occured while retrieving the user");
        UserCalendarCombinationRepository::store(driver().lock().unwrap().deref_mut(), &UserCalendarCombination::new(user.get_uuid(), calendar.uuid)).expect("Error occured while retrieving the user");
    }
    true
}
