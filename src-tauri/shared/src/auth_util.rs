extern crate bcrypt;

use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::MutexGuard;

use crate::db::db_actions::DbActions;
use crate::db::model::client::Client;
use crate::db::model::user::User;
use crate::db::repository::user_repository::UserRepository;
use crate::errors::error_messages::{
    BCRYPT_DECODING_ERR, USER_ALREADY_EXISTING_ERR, USER_NOT_FOUND_ERR,
};
use crate::runtime_objects::{driver, set_app_handle, set_current_user};
use bcrypt::{hash, verify, DEFAULT_COST};
use pg_driver::PgDriver;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_uuid: Uuid,
    pub client_uuid: Uuid,
}

pub struct AuthUtil;

impl AuthUtil {
    /// Attempts logging in to the system with the given credentials.
    ///
    /// # Arguments
    /// * `app_handle` - Contains information about the current app state and allows calling js
    /// functions from the backend.
    /// * `email` - The email provided by the user.
    /// * `password` - The password provided by the user.
    /// * `remember` - A flag of whether a jwt authentication process should be started to
    /// automatically log the user in on future sessions from the same device.
    ///
    /// # Returns an error
    ///
    /// - USER_NOT_FOUND_ERR: If no user with the provided credentials was found.
    /// - BCRYPT_DECODING_ERR: If something went wrong while decoding tokens.
    ///
    /// ## If something fails, the user sees it via a toast notification.
    pub fn attempt_login(
        app_handle: Option<AppHandle>,
        email: String,
        password: String,
        _remember: bool,
    ) -> Result<(), &'static str> {
        if let Some(handle) = app_handle {
            set_app_handle(handle);
        }
        let mut driver = driver().lock().unwrap();
        let user_exists = UserRepository::is_existing(driver.deref_mut(), &email);
        let user = UserRepository::get_by_email(driver.deref_mut(), email)?;
        let user_pass = &user.get_password();

        if !user_exists {
            return Err(USER_NOT_FOUND_ERR);
        }

        match verify(password, user_pass) {
            Ok(password_matches) => {
                if !password_matches {
                    return Err(USER_NOT_FOUND_ERR);
                }
            }
            Err(_) => return Err(BCRYPT_DECODING_ERR),
        }

        //if remember {
        //    Self::create_persistent_session(&user, &client, driver)?;
        //}

        set_current_user(user);
        Ok(())
    }

    /// Attempts creating a new account with the given credentials.
    ///
    /// # Arguments
    /// * `app_handle` - Contains information about the current app state and allows calling js
    /// functions from the backend.
    /// * `username` - The nickname of the user.
    /// * `email` - The email provided by the user.
    /// * `password` - The password provided by the user.
    /// * `remember` - A flag of whether a jwt authentication process should be started to
    /// automatically log the user in on future sessions from the same device.
    ///
    /// # Returns an error
    ///
    /// * `USER_ALREADY_EXISTS_ERR` - If a user with the given credentials already exists.
    /// * `JWT_COOKIE_ERR` - If something went wrong creating a persistent session for the user.
    /// The user is prompted to try again.
    ///
    /// ## If something fails, the user sees it via a toast notification.
    pub fn attempt_signup(
        app_handle: AppHandle,
        username: String,
        email: String,
        password: String,
        _remember: bool,
    ) -> Result<(), &'static str> {
        set_app_handle(app_handle);

        // let (prv_key, pub_key) = PKIAuthenticationKey::new_ed25519_key_pair();
        // let private_key = CryptoService::encrypt_private_key(&prv_key, &password)

        

        let hashed_password = hash(password, DEFAULT_COST).unwrap();
        let user = User::new(username, (&*email).into(), hashed_password);
        let mut driver = driver().lock().unwrap();

        if UserRepository::is_existing(driver.deref_mut(), &email) {
            return Err(USER_ALREADY_EXISTING_ERR);
        }

        UserRepository::store(driver.deref_mut(), &user).unwrap();

        //if remember {
        //    Self::create_persistent_session(&user, driver)?;
        //}

        set_current_user(user);
        Ok(())
    }

    /// Logs the current user out and deletes any ongoing sessions.
    ///
    /// # Arguments
    /// * `token` - The JwtToken used in the session to destroy.
    ///
    /// # Returns an error
    ///
    /// * `Logout failed unexpectedly` - If an unexpected error happens while deleting the session.
    ///
    /// ## If something fails, the user sees it via a toast notification.
    /// TODO: Improve error handling
    pub fn logout(_token: String) -> Result<(), &'static str> {
        todo!();
        //let res = JwtTokenRepository::delete_spec_col(
        //    driver().lock().unwrap().deref_mut(),
        //    String::from("token"),
        //    token,
        //);

        //if let Ok(()) = res {
        //    Ok(())
        //} else {
        //    Err("Logout failed unexpectedly")
        //}
    }

    /// Checks if the given token corresponds to a valid existing session. If so, the user is free
    /// to go without logging in.
    ///
    /// # Arguments
    /// * `token` - The token that is supposed to correspond to a valid session.
    pub fn is_valid_session(_token: String) -> bool {
        todo!()
    }

    /// Creates a local session by generating a jwt token, then storing it once in the database
    /// and calling the frontend to populate a http cookie containing the token data.
    ///
    /// # Arguments
    /// * `user` - The user associated with the session.
    /// * `driver` - The psql driver needed to perform database operations.
    ///
    /// # Returns an error
    ///
    /// * `JWT_COOKIE_ERR` - If something went wrong creating a persistent session for the user.
    /// The user is prompted to try again.
    ///
    /// ## If something fails, the user sees it via a toast notification.
    fn create_persistent_session(
        _user: &User,
        _client: &Client,
        _driver: MutexGuard<PgDriver>,
    ) -> Result<(), &'static str> {
        Ok(())
    }
}
