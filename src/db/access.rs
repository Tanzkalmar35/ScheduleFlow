use std::env;

use skytable::{Config, Connection, query};

pub fn connect() -> anyhow::Result<Connection> {
    let user = env::var("SKYTABLE_DB_USER").expect("skytable user env variable must be set.")
        .parse::<&str>()?;
    let password = env::var("SKYTABLE_DB_PASS").expect("skytable password env variable must be set.")
        .parse::<&str>()?;
    Ok(
        Config::new_default(user, password.as_str()).connect()?
    )
}

/// Insert a new row into the database.
///
/// ## Arguments
///
/// conn - The connection to the database.
/// model - The model to insert the row into.
/// values - The values to insert into the row.
pub fn insert(mut conn: Connection, model: String, values: Vec<String>) {
    let query = query!("insert into scheduleflow.{}({})", model, values);
    conn.query_parse::<()>(&query).unwrap();
}
