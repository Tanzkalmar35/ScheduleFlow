use postgres::{Client, Config, NoTls};
use serde::de::Error;

/// The database driver for PostgreSQL.
pub(crate) struct PgDriver {
    /// The postgres configuration.
    config: Config,
    /// The postgres client.
    client: Option<Client>,
}

impl PgDriver {
    /// Sets up the database driver.
    pub fn setup(user: String, pass: String, name: String, address: String) -> Result<Self, Box<dyn Error>> {
        let mut config = Config::new();
        config.user(&user);
        config.password(&pass);
        config.dbname(&name);
        config.host_path(&address);
        Ok(Self {
            config,
            client: None,
        })
    }

    /// Initializes the database connection client.
    pub fn connect(&mut self) -> Result<&Self, Box<dyn Error>> {
        self.client = Some(self.config.connect(NoTls)?);
        Ok(self)
    }

    /// Executes a query on the database.
    pub fn exec(&mut self, query: &str) -> Result<(), Box<dyn Error>> {
        self.client.as_mut().unwrap().execute(query, &[])?;
        Ok(())
    }
}
