use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::db;
use crate::db::db_actions::Table;
use crate::db::pg_driver::PgDriver;

pub struct User {
    id: Option<i32>,
    lastname: String,
    firstname: String,
    address: String,
    city: String,
}

impl User {
    /// Contains the usable field names of a user excluding the id.
    pub(crate) const FIELD_NAMES: [&'static str; 4] = ["lastname", "firstname", "address", "city"];

    /// Provides the values of the user formatted as a vector of &str.
    fn vals(&self) -> Vec<&str> {
        vec![&self.lastname, &self.firstname, &self.address, &self.city]
    }

    /// Creates a new user.
    pub(crate) fn new(lastname: String, firstname: String, address: String, city: String) -> Self {
        Self {
            id: None,
            lastname,
            firstname,
            address,
            city,
        }
    }

    fn new_with_id(id: i32, lastname: String, firstname: String, address: String, city: String) -> Self {
        Self {
            id: Some(id),
            lastname,
            firstname,
            address,
            city,
        }
    }
}

#[async_trait]
impl Table for User {
    async fn store<'a>(&'a mut self, driver: Arc<Mutex<PgDriver>>) -> Result<(), Box<dyn std::error::Error>> {
        let cols = Vec::from(User::FIELD_NAMES);
        let vals = self.vals();
        let id = Self::insert(
            driver,
            "users",
            cols,
            vals,
        ).await;
        match id {
            Ok(id) => self.id = Some(id),
            Err(e) => eprintln!("Error inserting user {}", e)
        }
        Ok(())
    }

    fn retrieve(driver: PgDriver) -> Vec<db::table_users::User> {
        let cols = vec![String::from("*")];
        let condition = None;
        let res = Self::read(
            driver,
            "users",
            cols,
            condition,
        );
        todo!("Convert the result into a Vec<User>")
    }

    async fn update(&self, driver: PgDriver) -> Result<(), Box<dyn std::error::Error>> {
        Ok(match self.id {
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
                )
            }
            None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Cannot update user without id"))),
        }.await.expect("User update failed!"))
    }

    async fn remove(&self, driver: PgDriver) -> Result<(), Box<dyn std::error::Error>> {
        Ok(match self.id {
            Some(id) => {
                Self::delete(
                    driver,
                    "users",
                    id,
                )
            }
            None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Cannot update user without id"))),
        }.await.expect("User deletion failed!"))
    }
}
