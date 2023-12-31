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
trait Table {
    fn insert(&self) {
        todo!("Create a base implementation for inserting entries into tables")
    }
    fn read(&self) {
        todo!("Create a base implementation for reading entries from tables")
    }
    fn update(&self) {
        todo!("Create a base implementation for updating entries in tables")
    }
    fn delete(&self) {
        todo!("Create a base implementation for deleting entries from tables")
    }
}
