use pg_driver::PgDriver;
use postgres::Row;
use uuid::Uuid;

pub trait Table<M> {
    fn get_name() -> String;
    fn get_fk_uuid_name() -> String;
    fn get_fmt_cols() -> String;
    fn get_fmt_cols_no_id() -> String;
    fn get_fmt_vals(model: &M) -> String;
    fn get_fmt_vals_no_id(model: &M) -> String;
}

/// This module holds the base implementation for the CRUD operations
pub trait DbActions<M, R: Table<M>> {
    /// Inserts a new entry into a given table.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to insert into.
    /// * `cols` - The columns to insert into.
    /// * `vals` - The values to insert into the columns.
    fn insert(driver: &mut PgDriver, entry: &M) -> anyhow::Result<()> {
        let stmt = &format!(
            "INSERT INTO {} ({}) VALUES ({})",
            R::get_name(),
            R::get_fmt_cols(),
            R::get_fmt_vals(entry)
        );
        match driver.exec(stmt) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Queries a given table.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to query.
    /// * `cols` - The columns to query.
    /// * `condition` - The condition to query. Optional.
    fn read(driver: &mut PgDriver, table: &str, condition: Option<String>) -> Vec<Row> {
        let rows = match condition {
            Some(condition) => {
                let query = format!("SELECT * FROM {} WHERE {}", table, condition);
                driver
                    .exec(&query)
                    // Should not happen!
                    .expect("Query with condition failed")
            }
            None => driver
                .exec(&format!("SELECT * FROM {}", table))
                // Should not happen!
                .expect("Query without condition failed."),
        };

        rows
    }

    /// Updates a given entry.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to insert into.
    /// * `cols` - The columns to insert into.
    /// * `vals` - The values to insert into the columns.
    fn alter(driver: &mut PgDriver, entry: &M, uuid: Uuid) -> anyhow::Result<()> {
        let col_binding = R::get_fmt_cols_no_id();
        let val_binding = R::get_fmt_vals_no_id(&entry);
        let cols = col_binding.split(", ").collect::<Vec<&str>>();
        let vals = val_binding.split(',').collect::<Vec<&str>>();

        let update_stmt = cols
            .iter()
            .zip(vals.iter())
            .map(|(c, v)| format!("\"{}\" = {}", c, v))
            .collect::<Vec<_>>()
            .join(", ");

        driver
            .exec(&format!(
                "UPDATE {} SET {} WHERE uuid = '{}'",
                R::get_name(),
                update_stmt,
                uuid
            ))
            // Should not happen!
            .expect("Update failed.");
        Ok(())
    }

    /// Deletes an entry from a given table using the 'uuid' column.
    ///
    /// # Arguments
    /// * `driver` - The database driver.
    /// * `table` - The table to delete from.
    /// * `user_id` - The id of the user to delete.
    fn delete(driver: &mut PgDriver, uuid: Uuid) -> anyhow::Result<()> {
        driver
            .exec(&format!(
                "DELETE FROM {} WHERE uuid='{}'",
                R::get_name(),
                uuid
            ))
            // Should not happen!
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
    fn delete_spec_col(driver: &mut PgDriver, col: String, val: String) -> anyhow::Result<()> {
        driver
            .exec(&format!(
                "DELETE FROM {} WHERE {}='{}'",
                R::get_name(),
                col,
                val
            ))
            // Should not happen!
            .expect("Deletion failed.");
        Ok(())
    }

    fn query(driver: &mut PgDriver, stmt: String) -> anyhow::Result<Vec<Row>> {
        driver.exec(&stmt)
    }

    /// The table specific implementation for adding a new entry.
    fn store(driver: &mut PgDriver, model: &M) -> anyhow::Result<()>;

    /// The table specific implementation for editing an entry.
    fn update(driver: &mut PgDriver, model: &M) -> anyhow::Result<()>;

    /// The table specific implementation for removing an entry.
    fn remove(driver: &mut PgDriver, model: &M) -> anyhow::Result<()>;

    /// The table specific implementation for retrieving an entry.
    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<M>;
}
