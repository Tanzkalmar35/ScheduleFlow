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
    pub fn setup(user: String, pass: String, name: String, address: String) -> anyhow::Result<Self> {
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
    pub fn exec(&mut self, query: &str) -> anyhow::Result<()> {
        self.client.as_mut().unwrap().execute(query, &[])?;
        Ok(())
    }
}
