// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use std::ops::DerefMut;
use std::thread;

use dotenv::dotenv;
use icalendar::Component;
use tauri::{Manager, Runtime};

use crate::auth_util::{attempt_login, attempt_signup, logout};
use crate::db_actions::DbActions;
use crate::error_queue::ErrorQueue;
use crate::jwt_controller::is_valid_session;
use crate::runtime_objects::{driver, set_current_window, set_error_queue};

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
mod runtime_objects;
mod error_queue;

fn main() {

    init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![attempt_signup, attempt_login, logout, is_valid_session, set_current_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init() {
    dotenv().ok();
    let error_queue = ErrorQueue::new();
    set_error_queue(error_queue);

    thread::spawn(move || {
        let driver = driver();
    });
}
