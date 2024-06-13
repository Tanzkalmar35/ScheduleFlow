use dotenv::dotenv;
use postgres::{Client, NoTls, Row};
use serde::{Deserialize, Deserializer, Serialize};
use crate::errors::ENV_VAR_NOT_SET;

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
        let client = Client::connect(&self.url, NoTls).expect("Cannot establish connection to the database.");
        self.client = Some(client);
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
            None => Err(anyhow::anyhow!("Database client is not connected.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::driver;
    use crate::pg_driver::PgDriver;

    #[test]
    pub fn test_db_connection() {
        assert!(driver().lock().is_ok())
    }
}
