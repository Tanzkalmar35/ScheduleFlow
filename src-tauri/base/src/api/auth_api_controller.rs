use shared::{
    auth_util::AuthUtil, db::repository::user_repository::UserRepository, runtime_objects::driver,
};
use std::ops::{Deref, DerefMut};
use tauri::AppHandle;

#[tauri::command]
pub(crate) fn attempt_login(
    app_handle: AppHandle,
    email: String,
    password: String,
    remember: bool,
) -> Result<(), &'static str> {
    AuthUtil::attempt_login(Some(app_handle), email, password, remember)
}

#[tauri::command]
pub(crate) fn attempt_signup(
    app_handle: AppHandle,
    username: String,
    email: String,
    password: String,
    remember: bool,
) -> Result<(), &'static str> {
    AuthUtil::attempt_signup(app_handle, username, email, password, remember)
}

#[tauri::command]
pub(crate) fn user_exists(email: &str) -> bool {
    println!("Email: {}", email);
    UserRepository::is_existing(driver().lock().unwrap().deref_mut(), email)
}

#[tauri::command]
pub(crate) fn logout(token: String) -> Result<(), &'static str> {
    AuthUtil::logout(token)
}

#[tauri::command]
pub(crate) fn is_valid_session() -> bool {
    AuthUtil::is_valid_session(driver().lock().unwrap().deref_mut())
}
