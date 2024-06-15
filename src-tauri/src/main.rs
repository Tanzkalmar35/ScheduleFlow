// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use std::ops::DerefMut;
use std::sync::{Arc, Mutex, OnceLock};
use dotenv::dotenv;
use icalendar::Component;
use once_cell::sync::{Lazy, OnceCell};
use table_users::User;
use tauri::{Manager, Runtime};

use crate::db_actions::DbActions;
use crate::jwt_controller::is_valid_session;
use crate::auth_util::{attempt_login, logout};
use crate::pg_driver::PgDriver;

#[path = "calendar/icalendar_util.rs"]
mod icalendar_util;

#[path = "db/tables/table_users.rs"]
mod table_users;
#[path = "db/tables/table_calendars.rs"]
mod table_calendars;
#[path = "db/tables/table_components.rs"]
mod table_components;
#[path = "db/tables/table_properties.rs"]
mod table_properties;
#[path = "db/tables/table_combinations.rs"]
mod table_combinations;
#[path = "db/tables/table_jwt_tokens.rs"]
mod table_jwt_tokens;
#[path = "db/db_actions.rs"]
mod db_actions;
#[path = "db/pg_driver.rs"]
mod pg_driver;
#[path = "db/tables/adapter.rs"]
mod adapter;

mod auth_util;
mod errors;
mod jwt_controller;

fn driver() -> &'static Mutex<PgDriver> {
    static PG_DRIVER: OnceLock<Mutex<PgDriver>> = OnceLock::new();
    PG_DRIVER.get_or_init(|| {
        let mut driver = PgDriver::setup();
        driver.connect();
        Mutex::new(driver)
    })
}

pub static CURRENT_USER: OnceCell<Mutex<Option<User>>> = OnceCell::new();

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

fn main() {
    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![attempt_login, logout, is_valid_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
