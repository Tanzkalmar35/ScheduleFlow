use dotenv::dotenv;
use postgres::{Client, NoTls, Row};

/// The database driver for PostgreSQL.
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
        dotenv().ok();
        let name = std::env::var("PSQL_NAME").expect("PSQL_NAME must be set.");
        let user = std::env::var("PSQL_USER").expect("PSQL_USER must be set.");
        let pass = std::env::var("PSQL_PASS").expect("PSQL_PASS must be set.");
        let address = std::env::var("PSQL_IP").expect("PSQL_IP must be set.");
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
        let client =
            Client::connect(&self.url, NoTls)?;
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
