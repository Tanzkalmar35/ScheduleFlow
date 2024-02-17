use dotenv::dotenv;
use postgres::Client;

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
    pub fn setup() -> anyhow::Result<Self> {
        dotenv().ok();
        let name = std::env::var("PSQL_NAME").expect("PSQL_NAME must be set.");
        let user = std::env::var("PSQL_USER").expect("PSQL_USER must be set.");
        let pass = std::env::var("PSQL_PASS").expect("PSQL_PASS must be set.");
        let address = std::env::var("PSQL_IP").expect("PSQL_IP must be set.");
        let url = format!("postgres://{}:{}@{}/{}", user, pass, address, name);
        Ok(
            Self {
                user,
                pass,
                name,
                address,
                url,
                client: None,
            }
        )
    }

    /// Initializes the database connection client.
    pub fn connect(&mut self) -> anyhow::Result<&mut Self> {
        self.client = Some(Client::connect(&self.url, postgres::NoTls)?);
        Ok(self)
    }

    /// Executes a query on the database.
    pub fn exec(&mut self, query: &str) -> anyhow::Result<Vec<postgres::Row>> {
        match self.client.as_mut() {
            Some(client) => {
                let rows = client.query(query, &[])?;
                Ok(rows)
            }
            None => Err(anyhow::anyhow!("Database client is not connected.")),
        }
    }

    /// Queries the database.
    pub fn query(&mut self, query: &str) -> anyhow::Result<Vec<postgres::Row>> {
        let rows = self.client.as_mut().unwrap().query(query, &[])?;
        Ok(rows)
    }
}
