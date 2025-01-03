use postgres::{Client, NoTls, Row};

// use crate::errors::error_impl::no_database_connection_error::NoDatabaseConnectionError;
// use crate::runtime_objects::get_error_queue;

/// The database driver for PostgreSQL.
#[derive(Default)]
pub struct PgDriver {
    url: String,
    /// The postgres client.
    client: Option<Client>,
}

impl PgDriver {
    /// Sets up the database driver.
    pub fn setup() -> Self {
        let name = std::env::var("PSQL_NAME").expect("PSQL NAME NOT SET");
        let user = std::env::var("PSQL_USER").expect("PSQL USER NOT SET");
        let pass = std::env::var("PSQL_PASS").expect("PSQL PASS NOT SET");
        let address = std::env::var("PSQL_IP").expect("PSQL ADDRESS NOT SET");
        let url = format!("postgres://{}:{}@{}/{}", user, pass, address, name);
        Self { url, client: None }
    }

    /// Initializes the database connection client.
    ///
    /// # Errors
    /// If an error occurs during executing this function, like if there's no db connection, it
    /// gets automatically handled by the error queue.
    pub fn connect(&mut self) -> &mut Self {
        let conn = Client::connect(&self.url, NoTls);

        if let Ok(client) = conn {
            self.client = Some(client);
        } else {
            // Handle that...
            // let err = NoDatabaseConnectionError::new();
            // get_error_queue().enqueue(err);
        }

        self
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
                // Handle that...
                // let err = NoDatabaseConnectionError::new();
                // get_error_queue().enqueue(err);
            }
        }
        Ok(rows)
    }
}

