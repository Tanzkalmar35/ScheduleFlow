extern crate bcrypt;

use std::error::Error;
use std::fmt::{Debug, format};
use tauri::{Window};
use std::ops::{Deref, DerefMut};
use crate::table_users::User;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::db_actions::DbActions;
use crate::driver;
use crate::errors::{SUCCESS, USER_ALREADY_EXISTING_ERR};
use crate::jwt_controller::generate_jwt;
use crate::pg_driver::PgDriver;

#[tauri::command]
pub fn attempt_login(window: Window, username: String, email: String, password: String, remember: bool) -> Result<(), &'static str> {
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let user = User::new(username,( &*email).into(), hashed_password);

    if (User::is_existing(driver().lock().unwrap().deref_mut(), email.as_str())) {
        return Err(USER_ALREADY_EXISTING_ERR);
    }

    if let Err(e) = user.store(driver().lock().unwrap().deref_mut()) {
        println!("Storing user failed: {}", e)
    };

    if remember {
        let token = generate_jwt(user.uuid);
        token.store(driver().lock().unwrap().deref_mut());

        if let Err(e) = window.emit("setJwtCookie", token.token) {
            println!("Error setting a http jwt cookie: {}", e);
            return Err("Error setting a http jwt cookie");
        }
    }

    // TODO: Redirect to home page

    return Ok(());
}
