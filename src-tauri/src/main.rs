// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use icalendar::Calendar;
use crate::db_actions::Table;
use crate::icalendar_util::ICalendarUtil;
use crate::pg_driver::PgDriver;
use crate::table_calendars::ICalendar;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[path="calendar/icalendar_util.rs"]
mod icalendar_util;

#[path="db/table_users.rs"]
mod table_users;
#[path="db/table_calendars.rs"]
mod table_calendars;
#[path="db/db_actions.rs"]
mod db_actions;
#[path= "db/pg_driver.rs"]
mod pg_driver;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
