use std::collections::HashMap;

use crate::pg_driver::PgDriver;
use crate::table_users::User;

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
    fn insert(driver: &mut PgDriver, table: &str, cols: Vec<&str>, vals: Vec<&str>) -> anyhow::Result<()> {
        let cols = cols.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", ");
        let vals = vals.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ");
        let stmt = &format!("INSERT INTO {} ({}) VALUES ({}) RETURNING userid", table, cols, vals);
        return match driver.exec(stmt) {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        };
    }

    /// Queries a given table.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to query.
    /// * `cols` - The columns to query.
    /// * `condition` - The condition to query. Optional.
    fn read(mut driver: PgDriver, table: &str, mut cols: Vec<String>, condition: Option<String>) -> anyhow::Result<Vec<Row>> {
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
    fn alter(driver: &mut PgDriver, table: &str, cols: Vec<&str>, vals: Vec<&str>, id: String) -> anyhow::Result<()> {
        let update_stmt = cols.iter().zip(vals.iter()).map(|(c, v)|
            format!("\"{}\" = '{}'", c, v)).collect::<Vec<_>>().join(", ");
        driver.exec(&format!("UPDATE {} SET {} WHERE userid = '{}'", table, update_stmt, id))
            .expect("Update failed.");
        Ok(())
    }

    /// Deletes an entry from a given table.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to delete from.
    /// * `user_id` - The id of the user to delete.
    fn delete(driver: &mut PgDriver, table: &str, user_id: String) -> anyhow::Result<()> {
        driver.exec(&format!("DELETE FROM {} WHERE userid='{}'", table, user_id))
            .expect("Deletion failed.");
        Ok(())
    }

    /// The table specific implementation for adding a new entry.
    fn store(&mut self, driver: &mut PgDriver) -> anyhow::Result<()>;

    /// The table specific implementation for retrieving an entry.
    /// As of now, there is no use case for this.
    //fn retrieve(driver: PgDriver) -> Vec<User>;

    /// The table specific implementation for editing an entry.
    fn update(&self, driver: &mut PgDriver) -> anyhow::Result<()>;

    /// The table specific implementation for removing an entry.
    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()>;
}
