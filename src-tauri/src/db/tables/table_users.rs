use uuid::Uuid;

use crate::db_actions::{DbActions, Table};
use crate::pg_driver::PgDriver;

#[derive(Debug)]
pub struct User {
    uuid: Uuid,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) email: String,
}

impl User {

    /// Creates a new user and prepares the raw values into values ready to be stored.
    pub(crate) fn new(username: String, password: String, email: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username,
            password,
            email,
        }
    }

    /// Creates a User object from values that already exist in the db.
    pub(crate) fn from(uuid: Uuid, username: String, password: String, email: String) -> Self {
        Self {
            uuid,
            username,
            password,
            email,
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

    fn retrieve(_driver: &mut PgDriver, _cols: Vec<String>, _condition: Option<String>) -> Vec<Self::Item> {
        todo!()
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
