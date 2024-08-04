extern crate bcrypt;

use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::MutexGuard;

use bcrypt::{hash, verify, DEFAULT_COST};
use tauri::Window;

use crate::db::db_actions::DbActions;
use crate::db::pg_driver::PgDriver;
use crate::db::tables::table_jwt_tokens::JwtToken;
use crate::db::tables::table_users::User;
use crate::errors::error_messages::{
    BCRYPT_DECODING_ERR, JWT_COOKIE_ERR, USER_ALREADY_EXISTING_ERR, USER_NOT_FOUND_ERR,
};
use crate::jwt_controller::generate_jwt;
use crate::runtime_objects::{driver, get_current_window, set_current_user, set_current_window};

#[tauri::command]
pub fn attempt_login(
    window: Window,
    email: String,
    password: String,
    remember: bool,
) -> Result<(), &'static str> {
    set_current_window(window);

    let mut driver = driver().lock().unwrap();
    let user_exists = User::is_existing(driver.deref_mut(), &email);
    let user = User::get_by_email(driver.deref_mut(), email)?;
    let user_pass = &user.get_password();

    match verify(password, user_pass) {
        Ok(password_matches) => {
            if !password_matches {
                return Err(USER_NOT_FOUND_ERR);
            }
        }
        Err(_) => return Err(BCRYPT_DECODING_ERR),
    }

    if remember {
        populate_jwt_cookie(&user, driver)?;
    }

    set_current_user(user);
    Ok(())
}

#[tauri::command]
pub fn attempt_signup(
    window: Window,
    username: String,
    email: String,
    password: String,
    remember: bool,
) -> Result<(), &'static str> {
    set_current_window(window);

    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let user = User::new(username, (&*email).into(), hashed_password);
    let mut driver = driver().lock().unwrap();

    if User::is_existing(driver.deref_mut(), &email) {
        return Err(USER_ALREADY_EXISTING_ERR);
    }

    user.store(driver.deref_mut()).unwrap();

    if remember {
        populate_jwt_cookie(&user, driver)?;
    }

    set_current_user(user);
    Ok(())
}

fn populate_jwt_cookie(user: &User, mut driver: MutexGuard<PgDriver>) -> Result<(), &'static str> {
    let token = generate_jwt(user.uuid);
    token.store(driver.deref_mut());

    get_current_window()
        .unwrap()
        .emit("setJwtCookie", token.token)
        .map_err(|_| JWT_COOKIE_ERR)?;
    Ok(())
}

#[tauri::command]
pub fn logout(token: String) -> Result<(), &'static str> {
    JwtToken::delete_spec_col::<JwtToken>(
        driver().lock().unwrap().deref_mut(),
        String::from("token"),
        token,
    );

    Ok(())
}
