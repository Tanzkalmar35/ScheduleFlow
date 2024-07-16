use postgres::{Client, NoTls, Row};
use serde::{Deserialize, Deserializer, Serialize};
use tracing::{error, info};
use crate::errors::error_messages::{ENV_VAR_NOT_SET, ERROR_QUEUE_NOT_INITIALIZED_ERR};
use crate::runtime_objects::get_error_queue;
use crate::errors::error_impl::no_database_connection_error;
use crate::errors::error_impl::no_database_connection_error::NoDatabaseConnectionError;

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

        match conn {
            Ok(client) => {
                self.client = Some(client);
                info!("Database connection up and running!")
            }
            Err(e) => {
                error!("No database connection could be established");
                let err = NoDatabaseConnectionError::new();
                get_error_queue().enqueue(err);
            }
        }

        Ok(self)
    }

    /// Executes a query on the database.
    ///
    /// # Returns
    /// The affected Rows, if there are any. If not, an Error.
    pub fn exec(&mut self, query: &str) -> anyhow::Result<Vec<Row>> {
        let mut rows: Vec<Row> = vec![];
        match self.client.as_mut() {
            Some(client) => {
                rows = client.query(query, &[])?;
            }
            None => {
                let err = NoDatabaseConnectionError::new();
                get_error_queue().enqueue(err);
            }
        }
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime_objects::driver;

    #[test]
    pub fn test_db_connection() {
        assert!(driver().lock().is_ok())
    }
}
