use postgres::Row;
use uuid::Uuid;

use crate::pg_driver::PgDriver;

pub trait TableCombination {}

pub trait Table {
    /// Returns the name of the table.
    fn get_name() -> String;
    /// Returns a for a psql expression formatted String containing all columns of the table.
    fn get_fmt_cols() -> String;
    /// The name of the uuid column if referenced as fk.
    ///
    /// # To be fixed:
    /// Although this concept works, it's not ideal.
    ///
    /// Todo: Find a better way to obtain these names of tables
    fn get_fk_uuid_name() -> String;
    /// Returns a for a psql expression formatted String containing all columns except for the id
    /// field of the table.
    fn get_fmt_cols_no_id() -> String;
    /// Returns a for a psql expression formatted String containing all values of the table.
    fn get_fmt_vals(&self) -> String;
    /// Returns a for a psql expression formatted String containing all values except for the id
    /// field of the table.
    fn get_fmt_vals_no_id(&self) -> String;
}

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
pub trait DbActions {
    /// The type the DbActions got implemented for.
    type Item;

    /// Inserts a new entry into a given table.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to insert into.
    /// * `cols` - The columns to insert into.
    /// * `vals` - The values to insert into the columns.
    fn insert<E: Table>(driver: &mut PgDriver, entry: E) -> anyhow::Result<()> {
        let stmt = &format!(
            "INSERT INTO {} ({}) VALUES ({})",
            E::get_name(),
            E::get_fmt_cols(),
            entry.get_fmt_vals()
        );
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
    fn read(driver: &mut PgDriver, table: &str, condition: Option<String>) -> anyhow::Result<Vec<Row>> {
        let rows = match condition {
            Some(condition) => {
                let x = &format!("SELECT * FROM {} WHERE {}", table, condition);
                driver.exec(x).expect("Query with condition failed")
            }
            None => driver.exec(&format!("SELECT * FROM {}", table))
                .expect("Query without condition failed.")
        };

        Ok(rows)
    }

    /// Updates a given entry.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to insert into.
    /// * `cols` - The columns to insert into.
    /// * `vals` - The values to insert into the columns.
    fn alter<E: Table>(driver: &mut PgDriver, entry: E, uuid: Uuid) -> anyhow::Result<()> {
        let col_binding = E::get_fmt_cols_no_id();
        let val_binding = entry.get_fmt_vals_no_id();
        let cols = col_binding.split(", ").collect::<Vec<&str>>();
        let vals = val_binding.split(',').collect::<Vec<&str>>();

        let update_stmt = cols.iter().zip(vals.iter()).map(|(c, v)|
            format!("\"{}\" = {}", c, v)).collect::<Vec<_>>().join(", ");

        driver.exec(&format!("UPDATE {} SET {} WHERE uuid = '{}'", E::get_name(), update_stmt, uuid))
            .expect("Update failed.");
        Ok(())
    }

    /// Deletes an entry from a given table using the 'uuid' column.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to delete from.
    /// * `user_id` - The id of the user to delete.
    fn delete<E: Table>(driver: &mut PgDriver, uuid: Uuid) -> anyhow::Result<()> {
        driver.exec(&format!("DELETE FROM {} WHERE uuid='{}'", E::get_name(), uuid))
            .expect("Deletion failed.");
        Ok(())
    }

    /// Deletes an entry from a given table using a specific given column.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to delete from.
    /// * `col` - The name of the column the uuid is matched on.
    /// * `uuid` - The uuid of the entry to be deleted.
    fn delete_spec_col<E: Table>(driver: &mut PgDriver, col: String, uuid: Uuid) -> anyhow::Result<()> {
        driver.exec(&format!("DELETE FROM {} WHERE {}='{}'", E::get_name(), col, uuid))
            .expect("Deletion failed.");
        Ok(())
    }

    /// The table specific implementation for adding a new entry.
    fn store(&self, driver: &mut PgDriver) -> anyhow::Result<()>;

    /// The table specific implementation for editing an entry.
    fn update(&self, driver: &mut PgDriver) -> anyhow::Result<()>;

    /// The table specific implementation for removing an entry.
    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()>;

    /// The table specific implementation for retrieving an entry.
    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<Self::Item>;
}
