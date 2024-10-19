use std::ops::DerefMut;
use crate::auth_util::AuthUtil;
use tauri::AppHandle;
use crate::db::repository::user_repository::UserRepository;
use crate::runtime_objects::driver;

#[tauri::command]
pub(crate) fn attempt_login(
    app_handle: AppHandle,
    email: String,
    password: String,
    remember: bool,
) -> Result<(), &'static str> {
    AuthUtil::attempt_login(app_handle, email, password, remember)
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
pub(crate) fn is_valid_session(token: String) -> bool {
    AuthUtil::is_valid_session(token)
}
