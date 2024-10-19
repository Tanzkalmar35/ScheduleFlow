use std::ops::DerefMut;
use crate::db::db_actions::DbActions;
use crate::db::model::calendar::Calendar;
use crate::db::model::simple::simple_calendar::SimpleCalendar;
use crate::db::repository::calendar_repository::CalendarRepository;
use crate::db::service::calendar_service::CalendarService;
use crate::runtime_objects::{driver, get_current_user};

#[tauri::command]
pub fn get_calendar_of_current_user() -> Vec<SimpleCalendar> {
    CalendarService::get_user_calendars(get_current_user().lock().unwrap().as_ref().unwrap())
}

#[tauri::command]
pub fn store_new_calendar(calendar_name: String, users: Vec<>) -> anyhow::Result<()> {
    let calendar = Calendar::new(calendar_name);
    CalendarRepository::store(driver().lock().unwrap().deref_mut(), &calendar)
}
