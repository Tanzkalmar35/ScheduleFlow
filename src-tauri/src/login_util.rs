extern crate bcrypt;

use std::ops::{Deref, DerefMut};
use crate::table_users::User;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::db_actions::DbActions;
use crate::driver;
use crate::pg_driver::PgDriver;

#[tauri::command]
pub fn attempt_login(username: String, email: String, password: String) {
    let hashed_password = hash(&password, DEFAULT_COST).unwrap();
    let user = User::new(username, email, hashed_password);

    if let Err(e) = user.store(driver().lock().unwrap().deref_mut()) {
        println!("Storing user failed: {}", e)
    }
}


