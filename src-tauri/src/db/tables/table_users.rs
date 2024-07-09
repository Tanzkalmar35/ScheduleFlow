use std::ops::{Deref, DerefMut};

use uuid::Uuid;

use crate::db::db_actions::{DbActions, Table};
use crate::db::pg_driver::PgDriver;
use crate::errors::error_messages::USER_NOT_FOUND_ERR;

#[derive(Debug, Clone)]
pub struct User {
    pub(crate) uuid: Uuid,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) email: String,
}

impl User {

    /// Creates a new user and prepares the raw values into values ready to be stored.
    pub(crate) fn new(username: String, email: String, password: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username,
            password,
            email,
        }
    }

    /// Creates a User object from values that already exist in the db.
    pub(crate) fn from(uuid: Uuid, username: String, email: String, password: String) -> Self {
        Self {
            uuid,
            username,
            password,
            email,
        }
    }

    /// Checks if a user with a given email already exists.
    ///
    /// # Returns
    /// True, if there is a user with the given email, otherwise false.
    pub(crate) fn is_existing(driver: &mut PgDriver, email: &str) -> bool {
        let condition = format!("email = '{}'", email);

        let res = Self::retrieve(driver, Some(condition));

        !res.is_empty()
    }

    pub(crate) fn get_password(&self) -> &String {
        &self.password
    }

    pub(crate) fn get_by_email(driver: &mut PgDriver, email: String) -> Result<Self, &'static str> {
        let condition = format!("email = '{}'", email);
        let user_opt = User::retrieve(driver, Some(condition)).get(0).cloned();
        if let Some(user) = user_opt {
            Ok(user)
        } else {
            Err(USER_NOT_FOUND_ERR)
        }
    }
}

impl Table for &User {
    fn get_name() -> String {
        String::from("users")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, username, password, email")
    }

    fn get_fk_uuid_name() -> String {
        String::from("user_uuid")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("username, password, email")
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}', '{}', '{}', '{}'", &self.uuid, &self.username, &self.password, &self.email)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        format!("'{}', '{}', '{}'", &self.username, &self.password, &self.email)
    }
}

impl DbActions for User {

    type Item = User;

    /// Stores the current user.
    ///
    /// # Example usage
    ///
    ///     // Creating the user
    ///     let mut user = User::new(
    ///         String::from("SOME_USERNAME"),
    ///         String::from("SOME_PASSWORD"),
    ///         String::from("SOME_EMAIL"),
    ///     );
    ///
    ///     // Creating the driver
    ///     let mut driver = PgDriver::setup();
    ///
    ///     // Attempting to establish a db connection
    ///     match driver.connect() {
    ///         Ok(_) => {
    ///             println!("Driver db connection succeeded.");
    ///             // Storing the user
    ///             if user.store(&mut driver).is_ok() {
    ///                 println!("User {:?} successfully stored.", user);
    ///             } else {
    ///                 eprintln!("User {:?} couldn't be stored.", user);
    ///             }
    ///         },
    ///         Err(err) => eprintln!("Driver db connection failed: {}", err),
    ///     }
    ///
    /// # Errors
    ///
    /// Returns an error if the insertion process fails.
    fn store(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::insert(
            driver,
            self
        )
    }

    /// Updates the current user to the values the user object currently holds.
    ///
    /// # Example usage
    ///
    ///     // Creating the user
    ///     let mut user = User::new(
    ///         String::from("SOME_USERNAME"),
    ///         String::from("SOME_PASSWORD"),
    ///         String::from("SOME_EMAIL"),
    ///     );
    ///
    ///     // Creating the driver
    ///     let mut driver = PgDriver::setup();
    ///
    ///     // Attempting to establish a db connection
    ///     match driver.connect() {
    ///         Ok(_) => {
    ///             println!("Driver db connection succeeded.");
    ///             // Storing the user
    ///             if user.store(&mut driver).is_ok() {
    ///                 // Updating the user object's values
    ///                 user.username = String::from("SOME_OTHER_USERNAME"); // Changing the username
    ///                 // Storing the updated user
    ///                 if user.update(&mut driver).is_ok() {
    ///                     println!("User {:?} successfully got updated.", user);
    ///                 } else {
    ///                     eprintln!("User {:?} could not be updated.", user);
    ///                 }
    ///             } else {
    ///                 eprintln!("User {:?} couldn't be stored.", user);
    ///             }
    ///         },
    ///         Err(err) => eprintln!("Driver db connection failed: {}", err);
    ///     }
    ///
    /// # Errors
    ///
    /// Returns an error if the update process fails.
    fn update(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::alter(
            driver,
            self,
            self.uuid,
        )
    }

    /// Updates the current user to the values the user object currently holds.
    ///
    /// # Example usage
    ///
    ///     // Creating the user
    ///     let mut user = User::new(
    ///         String::from("SOME_USERNAME"),
    ///         String::from("SOME_PASSWORD"),
    ///         String::from("SOME_EMAIL"),
    ///     );
    ///
    ///     // Creating the driver
    ///     let mut driver = PgDriver::setup();
    ///
    ///     // Attempting to establish a db connection
    ///     match driver.connect() {
    ///         Ok(_) => {
    ///             println!("Driver db connection succeeded.");
    ///             // Storing the user
    ///             if user.store(&mut driver).is_ok() {
    ///                 // Removing the user again
    ///                 if user.remove(&mut driver).is_ok() {
    ///                     println!("User {:?} successfully got removed.", user);
    ///                 } else {
    ///                     eprintln!("User {:?} could not be removed.", user);
    ///                 }
    ///             } else {
    ///                 eprintln!("User {:?} couldn't be stored.", user);
    ///             }
    ///         },
    ///         Err(err) => eprintln!("Driver db connection failed: {}", err),
    ///     }
    ///
    /// # Errors
    ///
    /// Returns an Error if the deletion process fails.
    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::delete::<&Self>(driver, self.uuid)
    }

    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<Self::Item> {
        let mut res: Vec<User> = vec![];

        if let Ok(rows) = Self::read(driver, "users", condition) {
            for row in rows {
                res.push(User {
                    uuid: row.get("uuid"),
                    username: row.get("username"),
                    password: row.get("password"),
                    email: row.get("email"),
                })
            };
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_user() {
        let mut res = false;

        let user = User::new(
            String::from("SOME_USERNAME"),
            String::from("SOME_PASSWORD"),
            String::from("SOME_EMAIL"),
        );

        // Creating the driver
        let mut driver = PgDriver::setup();

        // Attempting to establish a db connection
        match driver.connect() {
            Ok(_) => {
                println!("Driver db connection succeeded.");
                // Storing the user
                match user.store(&mut driver) {
                    Ok(_) => {
                        println!("User {:?} successfully stored.", user);
                        res = true
                    }
                    Err(e) => {
                        eprintln!("User couldn't be stored: {}", e);
                    }
                }
                if user.store(&mut driver).is_ok() {
                    println!("User {:?} successfully stored.", user);
                    res = true
                } else {
                    eprintln!("User {:?} couldn't be stored.", user);
                }
            }
            Err(err) => eprintln!("Driver db connection failed: {}", err),
        }

        assert_eq!(res, true);
    }

    #[test]
    fn test_update_user() {
        let mut res = false;

        let mut user = User::new(
            String::from("SOME_USERNAME"),
            String::from("SOME_PASSWORD"),
            String::from("SOME_EMAIL"),
        );

        // Creating the driver
        let mut driver = PgDriver::setup();

        // Attempting to establish a db connection
        match driver.connect() {
            Ok(_) => {
                println!("Driver db connection succeeded.");
                // Storing the user
                if user.store(&mut driver).is_ok() {
                    // Updating the user object's values
                    user.username = String::from("SOME_OTHER_USERNAME"); // Changing the username
                    // Storing the updated user
                    if user.update(&mut driver).is_ok() {
                        println!("User {:?} successfully got updated.", user);
                        res = true;
                    } else {
                        eprintln!("User {:?} could not be updated.", user);
                    }
                } else {
                    eprintln!("User {:?} couldn't be stored.", user);
                }
            },
            Err(err) => eprintln!("Driver db connection failed: {}", err),
        }

        assert_eq!(res, true);
    }

    #[test]
    fn test_remove_user() {
        let mut res = false;

        let user = User::new(
            String::from("SOME_USERNAME"),
            String::from("SOME_PASSWORD"),
            String::from("SOME_EMAIL"),
        );

        // Creating the driver
        let mut driver = PgDriver::setup();

        // Attempting to establish a db connection
        match driver.connect() {
            Ok(_) => {
                println!("Driver db connection succeeded.");
                // Storing the user
                if user.store(&mut driver).is_ok() {
                    // Removing the user again
                    if user.remove(&mut driver).is_ok() {
                        println!("User {:?} successfully got removed.", user);
                        res = true;
                    } else {
                        eprintln!("User {:?} could not be removed.", user);
                    }
                } else {
                    eprintln!("User {:?} couldn't be stored.", user);
                }
            }
            Err(err) => eprintln!("Driver db connection failed: {}", err),
        }

        assert_eq!(res, true);
    }
}
