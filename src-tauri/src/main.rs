// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
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
