use std::time::Duration;
use dotenv::dotenv;
use postgres::{Client, NoTls, Row};
use serde::{Deserialize, Deserializer, Serialize};
use crate::error_queue::Error;
use crate::errors::{ENV_VAR_NOT_SET, ERROR_QUEUE_NOT_INITIALIZED_ERR, NO_DB_CONNECTION_ERR};
use crate::runtime_objects::{CURRENT_WINDOW, ERROR_QUEUE, get_current_window, get_error_queue};

/// The database driver for PostgreSQL.
#[derive(Default)]
pub struct PgDriver {
    user: String,
    pass: String,
    name: String,
    address: String,
    url: String,
    /// The postgres client.
    client: Option<Client>,
}

impl PgDriver {

    /// Sets up the database driver.
    pub fn setup() -> Self {
        let name = std::env::var("PSQL_NAME").expect(ENV_VAR_NOT_SET);
        let user = std::env::var("PSQL_USER").expect(ENV_VAR_NOT_SET);
        let pass = std::env::var("PSQL_PASS").expect(ENV_VAR_NOT_SET);
        let address = std::env::var("PSQL_IP").expect(ENV_VAR_NOT_SET);
        let url = format!("postgres://{}:{}@{}/{}", user, pass, address, name);
        Self {
            user,
            pass,
            name,
            address,
            url,
            client: None,
        }
    }

    /// Initializes the database connection client.
    pub fn connect(&mut self) -> anyhow::Result<&mut Self> {
        let conn = Client::connect(&self.url, NoTls);

        if let Ok(client) = conn {
            self.client = Some(client);
        } else {
            let err = Error::new(
                NO_DB_CONNECTION_ERR.to_string(),
                Box::new(|| get_current_window().is_some()),
                Duration::from_secs(0)
            );
            if let Some(error_queue) = get_error_queue() {
                if let Some(error_queue_inner) = &*error_queue {
                    error_queue_inner.enqueue(err);
                } else {
                    panic!("{}", ERROR_QUEUE_NOT_INITIALIZED_ERR)
                }
            }
        }

        Ok(self)
    }

    /// Executes a query on the database.
    ///
    /// # Returns
    /// The affected Rows, if there are any. If not, an Error.
    pub fn exec(&mut self, query: &str) -> anyhow::Result<Vec<Row>> {
        match self.client.as_mut() {
            Some(client) => {
                let rows = client.query(query, &[]);
                Ok(rows?)
            }
            None => {
                let err = Error::new(
                    NO_DB_CONNECTION_ERR.to_string(),
                    Box::new(|| get_current_window().is_some()),
                    Duration::from_secs(0)
                );
                if let Some(error_queue) = get_error_queue() {
                    if let Some(error_queue_inner) = &*error_queue {
                        error_queue_inner.enqueue(err);
                    } else {
                        panic!("{}", ERROR_QUEUE_NOT_INITIALIZED_ERR)
                    }
                };
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pg_driver::PgDriver;
    use crate::runtime_objects::driver;

    #[test]
    pub fn test_db_connection() {
        assert!(driver().lock().is_ok())
    }
}
