// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::icalendar_util::ICalendarUtil;
use crate::pg_driver::PgDriver;
use crate::table_calendars::CalendarDAO;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
#[path= "db/tables/table_combinations.rs"]
mod table_combinations;
#[path="db/db_actions.rs"]
mod db_actions;
#[path= "db/pg_driver.rs"]
mod pg_driver;
#[path= "db/tables/adapter.rs"]
mod adapter;

fn main() {
    // let mut driver = PgDriver::setup();
    // if let Err(_) = driver.connect() {
    //     panic!("Driver conn failed")
    // }
    // let mut a = ICalendarUtil::init(CalendarDAO::new());
    // a.store(&mut driver);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
