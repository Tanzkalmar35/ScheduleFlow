// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use icalendar::{CalendarComponent, Component, Event, Property};
use crate::db_actions::DbActions;
use crate::icalendar_util::ICalendarUtil;
use crate::pg_driver::PgDriver;
use crate::table_calendars::CalendarDAO;
use crate::table_combinations::TableCombination;
use crate::table_components::ComponentDAO;
use crate::table_components::ComponentType::EVENT;
use crate::table_properties::PropertyDAO;

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
#[path="db/tables/table_combinations.rs"]
mod table_combinations;
#[path="db/db_actions.rs"]
mod db_actions;
#[path= "db/pg_driver.rs"]
mod pg_driver;
#[path= "db/tables/adapter.rs"]
mod adapter;

fn main() {
    let mut driver = PgDriver::setup();
    if let Err(_) = driver.connect() {
        panic!("Driver conn failed")
    }

    let cal = CalendarDAO::new();
    let cal_prop = PropertyDAO::new("NAME".to_string(), "CONVERTED_CAL".to_string());
    let event = ComponentDAO::new(EVENT);
    let event_prop = PropertyDAO::new("DATE".to_string(), "TODAY".to_string());
    let cal_component_comb: TableCombination<CalendarDAO, ComponentDAO> = TableCombination::new(cal.uuid, event.uuid);
    let cal_prop_comb: TableCombination<CalendarDAO, PropertyDAO> = TableCombination::new(cal.uuid, cal_prop.uuid);
    let event_prop_comb: TableCombination<ComponentDAO, PropertyDAO> = TableCombination::new(event.uuid, event_prop.uuid);

    cal.store(&mut driver);
    cal_prop.store(&mut driver);
    cal_prop_comb.store(&mut driver);

    event.store(&mut driver);
    event_prop.store(&mut driver);
    event_prop_comb.store(&mut driver);

    cal_component_comb.store(&mut driver);

    let cal = adapter::ICalendarAdapter::bundle_calendar(&mut driver, cal);

    println!("{}", cal);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
