use std::error::Error;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::calendar::create_event;
use crate::db::db_actions::Table;
use crate::db::pg_driver::PgDriver;

#[path = "cmd/cmd.rs"]
mod cmd;

#[path = "db/table_users.rs"]
mod table_users;

mod calendar;
mod config;
mod db;

#[tokio::main]
async fn main() {
    // We need to store the calendar in a file and get it out of it instead of
    // creating a new one every time the application starts.

    let matches = cmd::cmd().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            create_event(sub_matches)
                .expect("Error creating a new event");
        }
        Some(("user", sub_matches)) => {
            let x = sub_matches.get_one::<String>("firstname").expect("firstname not existing");
            let mut user = table_users::User::new(
                String::from("SOME_LASTNAME"),
                x.to_string(),
                String::from("SOME_ADDRESS"),
                String::from("SOME_CITY"),
            );
            match PgDriver::setup().await {
                Ok(pg_driver) => {
                    let driver = Arc::new(Mutex::new(pg_driver));
                    match driver.lock().await.connect().await {
                        // TODO: After this driver.lock() the driver is locked afterwards
                        Err(e) => {
                            eprintln!("Establishing connection failed: {}", e)
                        }
                        Ok(_) => {
                            let driver_clone = Arc::clone(&driver);
                            tokio::spawn(async move {
                                driver_clone.lock().await.conn.as_ref();
                            });
                            match user.store(Arc::clone(&driver)).await {
                                Ok(_) => {
                                    eprintln!("Successfully stored user.")
                                }
                                Err(e) => {
                                    eprintln!("Error storing user: {}", e)
                                }
                            }
                        }
                    };
                }
                Err(e) => { eprintln!("Error setting up the driver: {}", e); }
            }
        }
        Some(("config", sub_matches)) => {
            let user = config::User::new(sub_matches);
            println!("Successfully created new user {:?}", user);
        }
        _ => unimplemented!("This command is not implemented yet"),
    }
}
