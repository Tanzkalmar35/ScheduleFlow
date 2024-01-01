use crate::db::pg_driver::PgDriver;

/// This module contains the base implementation for the CRUD operations
///
/// Example usage:
/// ```
/// use crate::db::db_actions::Table;
///
/// struct User {
///     id: i32,
///     name: String,
///     email: String,
/// }
///
/// impl Table for User {
///     // User specific implementations
/// }
///
/// fn main() {
///    // Now you can access all base and user specific implementations
///    User::insert();
/// }
pub trait Table {
    /// Inserts a new entry into a given table table.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to insert into.
    /// * `cols` - The columns to insert into.
    /// * `vals` - The values to insert into the columns.
    fn insert(mut driver: PgDriver, table: &str, cols: Vec<&str>, vals: Vec<&str>) {
        let cols = cols.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", ");
        let vals = vals.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ");
        println!("cols: {}, vals: {}", cols, vals);
        driver.exec(&format!("INSERT INTO {} ({}) VALUES ({})", table, cols, vals))
            .expect("Insertion failed.");
    }

    /// Queries a given table.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to query.
    /// * `cols` - The columns to query.
    /// * `condition` - The condition to query. Optional.
    fn read(mut driver: PgDriver, table: String, cols: Vec<String>, condition: Option<String>) {
        match condition {
            None => {
                driver.exec(&format!("SELECT {:?} FROM {}", cols, table))
                    .expect("Query failed.");
            }
            Some(condition) => {
                driver.exec(&format!("SELECT {:?} FROM {} WHERE {}", cols, table, condition))
                    .expect("Query failed.");
            }
        }
    }

    fn update(&self) {
        todo!()
    }

    fn delete(&self) {
        todo!()
    }

    /// The table specific implementation for adding a new entry.
    fn store(&self, driver: PgDriver);

    /// The table specific implementation for retrieving an entry.
    fn retrieve();

    /// The table specific implementation for editing an entry.
    fn edit();

    /// The table specific implementation for removing an entry.
    fn remove();
}
