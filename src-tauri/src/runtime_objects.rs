use std::error::Error;
use std::sync::{Mutex, MutexGuard, OnceLock};

use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use tauri::Window;
use crate::db::model::user::User;
use crate::db::pg_driver::PgDriver;
use crate::errors::error_messages::ERROR_QUEUE_NOT_INITIALIZED_ERR;
use crate::errors::error_queue::ErrorQueue;

pub static CURRENT_USER: OnceCell<Mutex<Option<User>>> = OnceCell::new();
pub static ERROR_QUEUE: OnceCell<Mutex<Option<ErrorQueue>>> = OnceCell::new();

lazy_static! {
    pub static ref CURRENT_WINDOW: Mutex<Option<Window>> = Mutex::new(None);
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
    ERROR_QUEUE.get()
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

#[tauri::command]
pub fn set_current_window(window: Window) {
    *CURRENT_WINDOW.lock().unwrap() = Some(window);
}

pub fn get_current_window() -> Option<Window> {
    let current_window = CURRENT_WINDOW.lock().unwrap();
    current_window.clone()
}
