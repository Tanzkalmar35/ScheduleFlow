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
                Ok(mut driver) => {
                    if let Err(e) = driver.connect().await {
                        eprintln!("Error connecting to the database: {}", e);
                    } else {
                        match user.store(driver).await {
                            Ok(_) => println!("User stored successfully"),
                            Err(e) => { eprintln!("Error storing user: {}", e); }
                        }
                    }
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
