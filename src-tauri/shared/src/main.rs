// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use std::env;
use std::ops::DerefMut;
use std::thread;

use crate::errors::error_queue::ErrorQueue;
use crate::runtime_objects::{driver, set_app_handle, set_error_queue};
use dotenv::dotenv;
use tauri::{Manager, Runtime};

mod auth_util;
mod db;
mod errors;
mod runtime_objects;

fn main() {
    init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![set_app_handle,])
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
