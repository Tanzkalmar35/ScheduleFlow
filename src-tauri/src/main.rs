// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use icalendar::{Component};
use crate::db_actions::DbActions;
use crate::login_util::attempt_login;

#[path="calendar/icalendar_util.rs"]
mod icalendar_util;

#[path="db/tables/table_users.rs"]
mod table_users;
#[path="db/tables/table_calendars.rs"]
mod table_calendars;
#[path="db/tables/table_components.rs"]
mod table_components;
#[path="db/tables/table_properties.rs"]
mod table_properties;
#[path="db/tables/table_combinations.rs"]
mod table_combinations;
#[path="db/db_actions.rs"]
mod db_actions;
#[path= "db/pg_driver.rs"]
mod pg_driver;
#[path= "db/tables/adapter.rs"]
mod adapter;
mod login_util;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![attempt_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
