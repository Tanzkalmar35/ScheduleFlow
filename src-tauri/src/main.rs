// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use std::ops::DerefMut;
use std::thread;

use dotenv::dotenv;
use icalendar::Component;
use tauri::{Manager, Runtime};
use tracing::Level;
use crate::auth_util::{attempt_login, attempt_signup, logout};
use crate::errors::error_queue::ErrorQueue;
use crate::jwt_controller::is_valid_session;
use crate::runtime_objects::{driver, set_current_window, set_error_queue};

mod calendar;
mod db;
mod errors;

mod auth_util;
mod jwt_controller;
mod runtime_objects;

fn main() {
    tracing_subscriber::fmt::init();
    let _guard = tracing::span!(Level::INFO, "main");

    tracing::info!("Initializing the application");

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

    tracing::info!("Initialization complete")
}
