use std::env;

use dotenv::dotenv;
use skytable::{Config, Connection, Query, query};

pub fn connect() -> anyhow::Result<Connection> {
    dotenv().ok();
    let user = env::var("SKYTABLE_DB_USER").expect("skytable user env variable must be set.");
    let password = env::var("SKYTABLE_DB_PASS").expect("skytable password env variable must be set.");
    let connection = Config::new_default(user.as_str(), password.as_str()).connect().unwrap();
    println!("{:?}", connection);
    Ok(
        connection
    )
}

pub fn setup(mut conn: &mut Connection) {
    let mut query = query!("CREATE SPACE IF NOT EXISTS example");
    // Create space
    println!("{:?}", conn.query(&query).unwrap());

    query = query!("CREATE MODEL IF NOT EXISTS example.users(userId: uint64, username: string, password: string)");
    // Create model
    println!("{:?}", conn.query(&query).unwrap());
}

/// Insert a new row into the database.
///
/// ## Arguments
///
/// conn - The connection to the database.
/// model - The model to insert the row into.
/// values - The values to insert into the row.
pub fn insert(mut conn: &mut Connection, query: Query) {
    //let query = query!("insert into scheduleflow.{}({})", model, values);
    println!("{:?}", conn.query(&query).expect("Error inserting into the db"));
}

pub fn get(mut conn: &mut Connection, query: Query) {
    //let query = query!("select password from scheduleflow.users where username = ?", username);
    println!("{:?}", conn.query(&query).expect("Error reading from the db"));
}
