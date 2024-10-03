// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use std::env;
use std::ops::DerefMut;
use std::thread;

use dotenv::dotenv;
use tauri::{Manager, Runtime};

use crate::auth_util::{attempt_login, attempt_signup, logout};
use crate::calendar::icalendar_util::get_calendar_of_current_user;
use crate::errors::error_queue::ErrorQueue;
use crate::jwt_controller::is_valid_session;
use crate::runtime_objects::{driver, set_app_handle, set_error_queue};

mod calendar;
mod db;
mod errors;

mod auth_util;
mod jwt_controller;
mod runtime_objects;

fn main() {
    init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            attempt_signup,
            attempt_login,
            logout,
            is_valid_session,
            set_app_handle,
            get_calendar_of_current_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn init() {
    dotenv().ok();
    let error_queue = ErrorQueue::new();
    set_error_queue(error_queue);

    thread::spawn(move || {
        let driver = driver();
    });
}
