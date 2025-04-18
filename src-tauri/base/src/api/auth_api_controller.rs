use shared::{
    auth_util::AuthUtil, current::driver, db::repository::user_repository::UserRepository,
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
    AuthUtil::attempt_signup(Some(app_handle), username, email, password, remember)
}

#[tauri::command]
pub(crate) fn user_exists(email: &str) -> bool {
    UserRepository::is_existing(driver().lock().unwrap().deref_mut(), email)
}

#[tauri::command]
pub(crate) fn logout() -> Result<(), &'static str> {
    AuthUtil::logout(driver().lock().unwrap().deref_mut())
}

#[tauri::command]
pub(crate) fn is_valid_session() -> bool {
    AuthUtil::is_valid_session(driver().lock().unwrap().deref_mut())
}
