// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[path="table_users.rs"]
mod table_users;

#[path="db_actions.rs"]
mod db_actions;

#[path="pg_driver.rs"]
mod pg_driver;

use crate::db_actions::Table;
use crate::pg_driver::PgDriver;
use crate::table_users::User;

fn main() {

    let mut user = User::new(
        String::from("SOME_USERNAME"),
        String::from("SOME_PASSWORD"),
        String::from("SOME_EMAIL"),
    );

    let mut driver: PgDriver = PgDriver::setup().expect("Error setting up the driver.");

    match driver.connect() {
        Some(_) => {
            println!("Driver db connection succeeded.");
            match user.store(driver) {
                Ok(_) => eprintln!("Successfully stored user."),
                Err(e) => eprintln!("Error storing user: {}", e),
            };
        },
        None => eprintln!("Driver db connection failed."),
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Some(("user", sub_matches)) => {
//     let x = sub_matches.get_one::<String>("firstname").expect("firstname not existing");
//     let mut user = table_users::User::new(
//         String::from("SOME_LASTNAME"),
//         x.to_string(),
//         String::from("SOME_ADDRESS"),
//         String::from("SOME_CITY"),
//     );
//     match PgDriver::setup().await {
//         Ok(pg_driver) => {
//             let driver = Arc::new(Mutex::new(pg_driver));
//             let connection = match driver.lock().await.connect().await {
//         Ok(_) => {
//             println!("Driver db connection succeeded.");
//             true
//         }
//         Err(e) => {
//             eprintln!("Driver db connection failed: {}", e);
//             false
//         }
//     };
//     if connection {
//         let driver_clone = Arc::clone(&driver);
//         tokio::spawn(async move {
//             driver_clone.lock().await.conn.as_ref();
//                 });
//                 match user.store(Arc::clone(&driver)).await {
//                     Ok(_) => {
//                         eprintln!("Successfully stored user.")
//                     }
//                     Err(e) => {
//                         eprintln!("Error storing user: {}", e)
//                     }
//                 }
//             }
//         }
//         Err(e) => { eprintln!("Error setting up the driver: {}", e); }
//     }
// }
