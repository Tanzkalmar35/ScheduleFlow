use crate::db::model::simple::simple_calendar::SimpleCalendar;
use crate::db::service::calendar_service::CalendarService;
use crate::runtime_objects::get_current_user;

#[tauri::command]
pub fn get_calendar_of_current_user() -> Vec<SimpleCalendar> {
    CalendarService::get_user_calendars(get_current_user().lock().unwrap().as_ref().unwrap())
}
