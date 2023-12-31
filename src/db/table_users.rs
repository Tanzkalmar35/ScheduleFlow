use crate::db::db_actions::Table;
use crate::db::pg_driver::PgDriver;

struct User {
    id: i32,
    lastname: String,
    firstname: String,
    address: String,
    city: String,
}

impl Table for User {
    fn add(driver: PgDriver) {
        User::insert(
            driver,
            "users",
            vec!["lastname", "firstname", "address", "city"],
            vec!["Doe", "John", "1234 Main St", "Anytown, USA"],
        );
    }

    fn retrieve() {
        todo!()
    }

    fn edit() {
        todo!()
    }

    fn remove() {
        todo!()
    }
}
