extern crate bcrypt;

use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::MutexGuard;

use bcrypt::{hash, verify, DEFAULT_COST};
use tauri::{AppHandle, Emitter};

use crate::db::db_actions::DbActions;
use crate::db::model::jwt_token::JwtToken;
use crate::db::model::user::User;
use crate::db::pg_driver::PgDriver;
use crate::db::repository::jwt_token_repository::JwtTokenRepository;
use crate::db::repository::user_repository::UserRepository;
use crate::errors::error_messages::{
    BCRYPT_DECODING_ERR, JWT_COOKIE_ERR, USER_ALREADY_EXISTING_ERR, USER_NOT_FOUND_ERR,
};
use crate::jwt_controller::generate_jwt;
use crate::runtime_objects::{driver, get_app_handle, set_app_handle, set_current_user};

#[tauri::command]
pub fn attempt_login(
    app_handle: AppHandle,
    email: String,
    password: String,
    remember: bool,
) -> Result<(), &'static str> {
    set_app_handle(app_handle);

    let mut driver = driver().lock().unwrap();
    let user_exists = UserRepository::is_existing(driver.deref_mut(), &email);
    let user = UserRepository::get_by_email(driver.deref_mut(), email)?;
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
    app_handle: AppHandle,
    username: String,
    email: String,
    password: String,
    remember: bool,
) -> Result<(), &'static str> {
    set_app_handle(app_handle);

    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let user = User::new(username, (&*email).into(), hashed_password);
    let mut driver = driver().lock().unwrap();

    if UserRepository::is_existing(driver.deref_mut(), &email) {
        return Err(USER_ALREADY_EXISTING_ERR);
    }

    UserRepository::store(driver.deref_mut(), &user).unwrap();

    if remember {
        populate_jwt_cookie(&user, driver)?;
    }

    set_current_user(user);
    Ok(())
}

fn populate_jwt_cookie(user: &User, mut driver: MutexGuard<PgDriver>) -> Result<(), &'static str> {
    let token = generate_jwt(user.get_uuid());
    JwtTokenRepository::store(driver.deref_mut(), &token);

    get_app_handle()
        .unwrap()
        .emit("setJwtCookie", token.token)
        .map_err(|_| JWT_COOKIE_ERR)?;
    Ok(())
}

/// TODO: Improve error handling
#[tauri::command]
pub fn logout(token: String) -> Result<(), &'static str> {
    let res = JwtTokenRepository::delete_spec_col(
        driver().lock().unwrap().deref_mut(),
        String::from("token"),
        token,
    );

    if let Ok(()) = res {
        Ok(())
    } else {
        Err("Logout failed!")
    }
}
