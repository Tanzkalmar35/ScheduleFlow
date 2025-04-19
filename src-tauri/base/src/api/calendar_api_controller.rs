use shared::{
    current::{driver, get_current_user},
    db::{
        db_actions::DbActions,
        model::{
            calendar::Calendar, simple::simple_calendar::SimpleCalendar,
            user_calendar_combination::UserCalendarCombination,
        },
        repository::{
            calendar_repository::CalendarRepository,
            user_calendar_combination_repository::UserCalendarCombinationRepository,
            user_repository::UserRepository,
        },
        service::calendar_service::CalendarService,
    },
};

use crate::api::auth_api_controller::user_exists;
use std::ops::DerefMut;

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
        let user = UserRepository::get_by_email(driver().lock().unwrap().deref_mut(), email)
            .expect("Error occured while retrieving the user");
        UserCalendarCombinationRepository::store(
            driver().lock().unwrap().deref_mut(),
            &UserCalendarCombination::new(user.get_uuid(), calendar.uuid),
        )
        .expect("Error occured while retrieving the user");
    }
    true
}
