use std::string::ToString;
use uuid::Uuid;

use crate::db_actions::Table;
use crate::pg_driver::PgDriver;

#[derive(Debug)]
pub struct User {
    id: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) email: String,
}

impl User {
    /// Contains all field names of a user including the id.
    pub(crate) const FIELD_NAMES: [&'static str; 4] = ["id", "username", "password", "email"];

    /// Contains the editable field names of a user - excluding the id.
    pub(crate) const EDITABLE_FIELD_NAMES: [&'static str; 3] = ["username", "password", "email"];

    /// Provides the values of the user formatted as a vector of String.
    fn vals(&self) -> Vec<&str> {
        vec![&self.username, &self.password, &self.email]
    }

    /// Creates a new user and prepares the raw values into values ready to be stored.
    pub(crate) fn new(username: String, password: String, email: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            password,
            email,
        }
    }

    /// Creates a User object from values that already exist in the db.
    pub(crate) fn from(id: String, username: String, password: String, email: String) -> Self {
        Self {
            id,
            username,
            password,
            email,
        }
    }
}

impl Table for User {
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
    fn store(&mut self, driver: &mut PgDriver) -> anyhow::Result<()> {
        let cols = Vec::from(User::FIELD_NAMES);
        let mut vals = Vec::new();

        vals.push(self.id.as_str().clone());
        vals.extend(self.vals());

        Self::insert(
            driver,
            "users",
            cols,
            vals,
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
        let cols = Vec::from(User::EDITABLE_FIELD_NAMES);
        let vals = self.vals();
        let user_id = format!("{}", self.id);
        Self::alter(
            driver,
            "users",
            cols,
            vals,
            user_id,
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
        Self::delete(driver, "users", self.id.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use super::*;

    #[test]
    fn test_store_user() {
        let mut res = false;
        let start_time;
        let mut elapsed_time = Duration::from_millis(1);

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
                start_time = Instant::now();
                if user.store(&mut driver).is_ok() {
                    elapsed_time = start_time.elapsed();
                    println!("User {:?} successfully stored.", user);
                    res = true
                } else {
                    eprintln!("User {:?} couldn't be stored.", user);
                }
            }
            Err(err) => eprintln!("Driver db connection failed: {}", err),
        }

        println!("Inserting a new user took {:?}", elapsed_time);
        assert_eq!(res, true);
    }

    #[test]
    fn test_update_user() {
        let mut res = false;
        let start_time;
        let mut elapsed_time = Duration::from_millis(1);

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
                    start_time = Instant::now();
                    if user.update(&mut driver).is_ok() {
                        elapsed_time = start_time.elapsed();
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

        println!("Updating an user took {:?}", elapsed_time);
        assert_eq!(res, true);
    }

    #[test]
    fn test_remove_user() {
        let mut res = false;
        let start_time;
        let mut elapsed_time = Duration::from_millis(1);

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
                    // Removing the user again
                    start_time = Instant::now();
                    if user.remove(&mut driver).is_ok() {
                        elapsed_time = start_time.elapsed();
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

        println!("Deleting an user took {:?}", elapsed_time);
        assert_eq!(res, true);
    }
}
