extern crate bcrypt;

use crate::db_actions::DbActions;
use crate::errors::{BCRYPT_DECODING_ERR, SUCCESS, USER_ALREADY_EXISTING_ERR, USER_EMAIL_NOT_FOUND_ERR, USER_NOT_FOUND_ERR};
use crate::jwt_controller::generate_jwt;
use crate::pg_driver::PgDriver;
use crate::table_jwt_tokens::JwtToken;
use crate::table_users::User;
use crate::{driver, set_current_user, CURRENT_USER};
use bcrypt::{hash, verify, DEFAULT_COST};
use once_cell::sync::Lazy;
use std::error::Error;
use std::fmt::{format, Debug};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use tauri::{window, Window};

#[tauri::command]
pub fn attempt_login(
    window: Window,
    email: String,
    password: String,
    remember: bool
) -> Result<(), &'static str> {
    let user_exists = User::is_existing(driver().lock().unwrap().deref_mut(), email.as_str());

    if !user_exists {
        return Err(USER_EMAIL_NOT_FOUND_ERR);
    }

    let user = User::get_by_email(driver().lock().unwrap().deref_mut(), email).unwrap();
    let user_pass = &user.get_password();

    let password_matches = verify(
        password,
        user_pass
    );

    if let Err(e) = password_matches {
        return Err(BCRYPT_DECODING_ERR);
    }

    if password_matches.unwrap() {
        return Ok(());
    }

    Err(USER_NOT_FOUND_ERR)
}

#[tauri::command]
pub fn attempt_signup(
    window: Window,
    username: String,
    email: String,
    password: String,
    remember: bool,
) -> Result<(), &'static str> {
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let user = User::new(username, (&*email).into(), hashed_password);

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
    set_current_user(user);

    return Ok(());
}

#[tauri::command]
pub fn logout(token: String) -> Result<(), &'static str> {
    JwtToken::delete_spec_col::<JwtToken>(
        driver().lock().unwrap().deref_mut(),
        String::from("token"),
        token
    );

    Ok(())
}
