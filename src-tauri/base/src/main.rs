// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use std::env;
use std::ops::DerefMut;
use std::thread;

use api::{
    auth_api_controller::{attempt_login, attempt_signup, is_valid_session, logout, user_exists},
    calendar_api_controller::{get_calendar_of_current_user, store_new_calendar},
};
use shared::{
    auth_util,
    current::{
        driver, get_session_type, set_app_handle as set_shared_app_handle, set_error_queue,
        SessionType,
    },
    errors::error_queue::ErrorQueue,
};

use dotenv::dotenv;
use tauri::{AppHandle, Manager, Runtime};

mod api;

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
            user_exists,
            store_new_calendar,
        ])
        .on_window_event(|_, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => elim_session_if_temp(),
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn init() {
    dotenv().ok();
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let error_queue = ErrorQueue::new();
    set_error_queue(error_queue);

    thread::spawn(move || {
        let driver = driver();
    });
}

fn elim_session_if_temp() {
    if let SessionType::TEMPORARY = get_session_type() {
        auth_util::AuthUtil::logout(driver().lock().unwrap().deref_mut());
    }
}

#[tauri::command]
fn set_app_handle(app_handle: AppHandle) {
    set_shared_app_handle(app_handle);
}
