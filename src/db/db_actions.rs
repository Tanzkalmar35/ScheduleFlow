use std::collections::HashMap;

use crate::db::pg_driver::PgDriver;
use crate::db::table_users::User;

pub type Row = HashMap<String, String>;

/// This module holds the base implementation for the CRUD operations
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
///    // Now you can access all base- and user specific implementations
///    User::insert(args);
/// }
pub trait Table {
    /// Inserts a new entry into a given table.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to insert into.
    /// * `cols` - The columns to insert into.
    /// * `vals` - The values to insert into the columns.
    fn insert(mut driver: PgDriver, table: &str, cols: Vec<&str>, vals: Vec<&str>) -> Result<i32, Box<dyn std::error::Error>> {
        let cols = cols.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", ");
        let vals = vals.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ");
        println!("cols: {}, vals: {}", cols, vals);
        let rows = driver.exec(&format!("INSERT INTO {} ({}) VALUES ({}) RETURNING userid", table, cols, vals))
            .expect("Insertion failed.");
        Ok(rows.get(0).unwrap().get("userid"))
    }

    /// Queries a given table.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to query.
    /// * `cols` - The columns to query.
    /// * `condition` - The condition to query. Optional.
    fn read(mut driver: PgDriver, table: &str, mut cols: Vec<String>, condition: Option<String>) -> Result<Vec<Row>, Box<dyn std::error::Error>> {
        let mut res = vec![];

        if cols.contains(&"*".to_string()) && cols.len() == 1 {
            cols = User::FIELD_NAMES.iter().map(|fld| fld.to_string()).collect::<Vec<_>>();
        }

        let fmt_cols = cols.join(", ");

        let rows = match condition {
            Some(condition) => driver.query(&format!("SELECT {} FROM {} WHERE {}", fmt_cols, table, condition))
                .expect("Query with condition failed."),
            None => driver.query(&format!("SELECT {} FROM {}", fmt_cols, table))
                .expect("Query without condition failed.")
        };

        for row in &rows {
            let mut row_data = Row::new();
            for (i, col) in cols.iter().enumerate() {
                row_data.insert(col.clone(), row.get(i));
            }
            res.push(row_data);
        }
        Ok(res)
    }

    /// Updates a given entry.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to insert into.
    /// * `cols` - The columns to insert into.
    /// * `vals` - The values to insert into the columns.
    fn alter(mut driver: PgDriver, table: &str, cols: Vec<&str>, vals: Vec<&str>, condition: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let update_stmt = cols.iter().zip(vals.iter()).map(|(c, v)|
            format!("\"{}\" = '{}'", c, v)).collect::<Vec<_>>().join(", ");
        match condition {
            Some(condition) => {
                driver.exec(&format!("UPDATE {} SET {} WHERE {}", table, update_stmt, condition))
                    .expect("Update failed.")
            }
            None => {
                driver.exec(&format!("UPDATE {} SET {}", table, update_stmt))
                    .expect("Update failed.")
            }
        };
        Ok(())
    }

    /// Deletes an entry from a given table.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to delete from.
    /// * `user_id` - The id of the user to delete.
    fn delete(mut driver: PgDriver, table: &str, user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        driver.exec(&format!("DELETE FROM {} WHERE userid={}", table, user_id))
            .expect("Deletion failed.");
        Ok(())
    }

    /// The table specific implementation for adding a new entry.
    fn store(&mut self, driver: PgDriver);

    /// The table specific implementation for retrieving an entry.
    fn retrieve(driver: PgDriver) -> Vec<User>;

    /// The table specific implementation for editing an entry.
    fn update(&self, driver: PgDriver);

    /// The table specific implementation for removing an entry.
    fn remove(&self, driver: PgDriver);
}
