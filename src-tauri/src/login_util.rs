extern crate bcrypt;

use std::ops::{Deref, DerefMut};
use crate::table_users::User;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::db_actions::DbActions;
use crate::driver;
use crate::errors::{SUCCESS, USER_ALREADY_EXISTING_ERR};
use crate::pg_driver::PgDriver;

#[tauri::command]
pub fn attempt_login(username: String, email: String, password: String) -> Result<(), &'static str> {
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let user = User::new(username,( &*email).into(), hashed_password);

    if (User::is_existing(driver().lock().unwrap().deref_mut(), email.as_str())) {
        return Err(USER_ALREADY_EXISTING_ERR);
    }

    if let Err(e) = user.store(driver().lock().unwrap().deref_mut()) {
        println!("Storing user failed: {}", e)
    };

    return Ok(());
}


