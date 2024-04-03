use crate::db_actions::Table;
use crate::pg_driver::PgDriver;

pub struct User {
    id: Option<i32>,
    username: String,
    password: String,
    email: String,
}

impl User {
    /// Contains the usable field names of a user excluding the id.
    pub(crate) const FIELD_NAMES: [&'static str; 3] = ["username", "password", "email"];

    /// Provides the values of the user formatted as a vector of &str.
    fn vals(&self) -> Vec<&str> {
        vec![&self.username, &self.password, &self.email]
    }

    /// Creates a new user.
    pub(crate) fn new(username: String, password: String, email: String) -> Self {
        Self {
            id: None,
            username,
            password,
            email,
        }
    }
}

impl Table for User {
    fn store<'a>(&'a mut self, driver: PgDriver) -> Result<(), Box<dyn std::error::Error>> {
        let cols = Vec::from(User::FIELD_NAMES);
        let vals = self.vals();
        let id = Self::insert(
            driver,
            "users",
            cols,
            vals,
        );
        match id {
            Ok(id) => self.id = Some(id),
            Err(e) => eprintln!("Error inserting user {}", e)
        }
        Ok(())
    }

    fn retrieve(driver: PgDriver) -> Vec<User> {
        let cols = vec![String::from("*")];
        let condition = None;
        let _res = Self::read(
            driver,
            "users",
            cols,
            condition,
        );
        todo!("Convert the result into a Vec<User>")
    }

    fn update(&self, driver: PgDriver) -> Result<(), Box<dyn std::error::Error>> {
        Ok(
            match self.id {
                Some(id) => {
                    let cols = Vec::from(User::FIELD_NAMES);
                    let vals = self.vals();
                    let condition = format!("userid={}", id);
                    Self::alter(
                        driver,
                        "users",
                        cols,
                        vals,
                        Some(condition),
                    ).expect("Altering table users failed.");
                }
                None => { anyhow::anyhow!("Cannot alter a user without its id."); },
            }
        )
    }

    fn remove(&self, driver: PgDriver) -> Result<(), Box<dyn std::error::Error>> {
        Ok(
            return match self.id {
                Some(id) => {
                    Self::delete(
                        driver,
                        "users",
                        id,
                    )
                }
                None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Cannot update user without id"))),
            }
        )
    }
}
