use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::db::model::client::Client;
use crate::db::model::simple::simple_calendar::SimpleCalendar;
use crate::db::model::user::User;
use crate::errors::error_messages::ERROR_QUEUE_NOT_INITIALIZED_ERR;
use crate::errors::error_queue::ErrorQueue;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use pg_driver::PgDriver;
use tauri::AppHandle;

pub static CURRENT_CLIENT: OnceCell<Mutex<Option<Client>>> = OnceCell::new();
pub static CURRENT_USER: OnceCell<Mutex<Option<User>>> = OnceCell::new();
pub static ERROR_QUEUE: OnceCell<Mutex<Option<ErrorQueue>>> = OnceCell::new();
pub static CACHED_CALENDARS: OnceCell<Mutex<Vec<SimpleCalendar>>> = OnceCell::new();
pub static SESSION_TYPE: OnceCell<Mutex<SessionType>> = OnceCell::new();

lazy_static! {
    pub static ref APP_HANDLE: Mutex<Option<AppHandle>> = Mutex::new(None);
}

#[derive(Debug, Clone, Copy)]
pub enum SessionType {
    TEMPORARY,
    PERSISTENT,
    NONE,
}

pub fn driver() -> &'static Mutex<PgDriver> {
    static PG_DRIVER: OnceLock<Mutex<PgDriver>> = OnceLock::new();
    PG_DRIVER.get_or_init(|| {
        let mut driver = PgDriver::setup();
        driver.connect();
        Mutex::new(driver)
    })
}

pub fn set_current_user(user: User) {
    CURRENT_USER.get_or_init(|| Mutex::new(Some(user)));
}

pub fn get_current_user() -> &'static Mutex<Option<User>> {
    CURRENT_USER.get().expect("Current user is not initialized")
}

pub fn reset_current_user() {
    if let Some(user_mutex) = CURRENT_USER.get() {
        let mut user_option = user_mutex.lock().unwrap();
        *user_option = None;
    } else {
        panic!("Current user is not initialized");
    }
}

pub fn set_current_client(client: Client) {
    CURRENT_CLIENT.get_or_init(|| Mutex::new(Some(client)));
}

pub fn get_current_client() -> &'static Mutex<Option<Client>> {
    CURRENT_CLIENT
        .get()
        .expect("Current user is not initialized")
}

pub fn reset_current_client() {
    if let Some(client_mutex) = CURRENT_CLIENT.get() {
        let mut client_option = client_mutex.lock().unwrap();
        *client_option = None;
    } else {
        panic!("Current client is not initialized");
    }
}

pub fn set_error_queue(error_queue: ErrorQueue) {
    ERROR_QUEUE.get_or_init(|| Mutex::new(Some(error_queue)));
}

pub fn get_error_queue() -> ErrorQueue {
    // ERROR_QUEUE.get().map(|mutex| {
    //     // if let Some(error_queue) = mutex.lock().unwrap() {
    //     //     if let Some(error_queue_inner) = &*error_queue {
    //     //         return error_queue_inner;
    //     //     } else {
    //     //         panic!("{}", ERROR_QUEUE_NOT_INITIALIZED_ERR)
    //     //     }
    //     // }
    //     mutex.lock().unwrap()
    // })
    ERROR_QUEUE
        .get()
        .and_then(|mutex| mutex.lock().ok())
        .and_then(|guard| guard.as_ref().cloned())
        .expect(ERROR_QUEUE_NOT_INITIALIZED_ERR)
}

pub fn reset_error_queue() {
    if let Some(error_queue_mutex) = ERROR_QUEUE.get() {
        let mut error_queue_option = error_queue_mutex.lock().unwrap();
        *error_queue_option = None;
    } else {
        panic!("{}", ERROR_QUEUE_NOT_INITIALIZED_ERR);
    }
}

pub fn set_app_handle(app_handle: AppHandle) {
    *APP_HANDLE.lock().unwrap() = Some(app_handle);
}

pub fn get_app_handle() -> Option<AppHandle> {
    APP_HANDLE.lock().unwrap().clone()
}

pub fn cache_calendar(calendar: SimpleCalendar) {
    if CACHED_CALENDARS.get().is_none() {
        CACHED_CALENDARS.set(Mutex::new(Vec::new())).unwrap();
    }

    let mut cached_calendars = CACHED_CALENDARS.get().unwrap().lock().unwrap();
    cached_calendars.push(calendar);
}

pub fn get_cached_calendars() -> MutexGuard<'static, Vec<SimpleCalendar>> {
    CACHED_CALENDARS.get().unwrap().lock().unwrap()
}

pub fn set_session_type(session_type: SessionType) {
    if SESSION_TYPE.get().is_none() {
        SESSION_TYPE.set(Mutex::new(session_type)).unwrap();
    }

    let mut cur_type = SESSION_TYPE.get().unwrap().lock().unwrap();
    *cur_type = session_type;
}

pub fn get_session_type() -> SessionType {
    return *SESSION_TYPE.get().unwrap().lock().unwrap();
}
