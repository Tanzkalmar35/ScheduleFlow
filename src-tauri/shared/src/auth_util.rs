extern crate bcrypt;

use std::fmt::Debug;
use std::ops::DerefMut;

use crate::crypto::crypto_service::CryptoService;
use crate::crypto::secure_storage::SecureStorage;
use crate::db::db_actions::DbActions;
use crate::db::model::client::Client;
use crate::db::model::user::User;
use crate::db::repository::client_repository::ClientRepository;
use crate::db::repository::user_repository::UserRepository;
use crate::errors::error_messages::{
    BCRYPT_DECODING_ERR, USER_ALREADY_EXISTING_ERR, USER_NOT_FOUND_ERR,
};
use crate::runtime_objects::{driver, set_app_handle, set_current_user};
use bcrypt::{hash, verify, DEFAULT_COST};
use customs::bench_message;
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
    #[bench_message("Attempting login")]
    pub fn attempt_login(
        app_handle: Option<AppHandle>,
        email: String,
        password: String,
        remember: bool,
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

        if remember {
            Self::create_persistent_session(&user, driver.deref_mut())?;
        }

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
    #[bench_message("Attempting signup")]
    pub fn attempt_signup(
        app_handle: Option<AppHandle>,
        username: String,
        email: String,
        password: String,
        remember: bool,
    ) -> Result<(), &'static str> {
        if let Some(handle) = app_handle {
            set_app_handle(handle);
        }

        let hashed_password = hash(&password, DEFAULT_COST).unwrap();
        let user = User::new(username, (&*email).into(), hashed_password);
        let mut driver = driver().lock().unwrap();

        if UserRepository::is_existing(driver.deref_mut(), &email) {
            return Err(USER_ALREADY_EXISTING_ERR);
        }

        UserRepository::store(driver.deref_mut(), &user).unwrap();

        if remember {
            Self::create_persistent_session(&user, driver.deref_mut())?;
        }

        set_current_user(user);
        Ok(())
    }

    /// Logs the current user out and deletes any ongoing sessions.
    ///
    /// # Arguments
    ///
    /// * `token` - The JwtToken used in the session to destroy.
    ///
    /// # Returns an error
    ///
    /// * `Logout failed unexpectedly` - If an unexpected error happens while deleting the session.
    ///
    /// ## If something fails, the user sees it via a toast notification.
    /// TODO: Improve error handling
    #[bench_message("Logging out")]
    pub fn logout(driver: &mut PgDriver) -> Result<(), &'static str> {
        let user_email = SecureStorage::get_system_key(&String::from("user_email")).unwrap();
        let user = UserRepository::get_by_email(driver, user_email).unwrap();
        let user_clients =
            ClientRepository::retrieve(driver, Some(format!("user_uuid = '{}'", user.get_uuid())));
        for client in user_clients {
            if client.get_device_name() == whoami::devicename() {
                if let Err(e) = ClientRepository::remove(driver, &client) {
                    log::error!("{}", e);
                }
            }
        }
        Ok(())
    }

    /// Checks if the given token corresponds to a valid existing session. If so, the user is free
    /// to go without logging in.
    ///
    /// # Arguments
    /// * `token` - The token that is supposed to correspond to a valid session.
    #[bench_message("Validating session")]
    pub fn is_valid_session(driver: &mut PgDriver) -> bool {
        let user_email = SecureStorage::get_system_key(&String::from("user_email")).unwrap();
        let user = UserRepository::get_by_email(driver, user_email).unwrap();
        let user_clients =
            ClientRepository::retrieve(driver, Some(format!("user_uuid = '{}'", user.get_uuid())));
        let prv_key_str = SecureStorage::get_system_key(user.get_email());
        let decrypted_key = CryptoService::decrypt_private_key(
            &prv_key_str.unwrap().as_str(),
            &user.get_password(),
        );

        for client in user_clients {
            let sign_successful = CryptoService::attempt_sign(
                &decrypted_key.as_ref().unwrap().as_bytes().to_vec(),
                &client.get_pub_key().as_bytes().to_vec(),
            );
            if sign_successful {
                set_current_user(user);
                return true;
            };
        }

        false
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
    // TODO: Improve error handling
    #[bench_message("Creating a new persistent session")]
    fn create_persistent_session(user: &User, driver: &mut PgDriver) -> Result<(), &'static str> {
        let (prv_key, pub_key) = CryptoService::new_ed25519_key_pair();
        let private_key =
            CryptoService::encrypt_private_key(&prv_key, &user.get_password()).unwrap();

        assert!(SecureStorage::store_system_key(&private_key, &user.get_email()).is_ok());
        assert!(
            SecureStorage::store_system_key(&user.get_email(), &String::from("user_email")).is_ok()
        );

        let client = Client::new(whoami::devicename(), user.get_uuid(), pub_key);

        assert!(ClientRepository::store(driver, &client).is_ok());

        Ok(())
    }
}
