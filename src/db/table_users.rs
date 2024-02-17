use crate::db::db_actions::Table;
use crate::db::pg_driver::PgDriver;

pub struct User {
    id: Option<i32>,
    lastname: String,
    firstname: String,
    address: String,
    city: String,
}

impl User {
    /// Contains the usable field names of a user excluding the id.
    pub(crate) const FIELD_NAMES: [&'static str; 4] = ["lastname", "firstname", "address", "city"];

    /// Provides the values of the user formatted as a vector of &str.
    fn vals(&self) -> Vec<&str> {
        vec![&self.lastname, &self.firstname, &self.address, &self.city]
    }

    /// Creates a new user.
    pub(crate) fn new(lastname: String, firstname: String, address: String, city: String) -> Self {
        Self {
            id: None,
            lastname,
            firstname,
            address,
            city,
        }
    }

    //fn new_with_id(id: i32, lastname: String, firstname: String, address: String, city: String) -> Self {
    //    Self {
    //        id: Some(id),
    //        lastname,
    //        firstname,
    //        address,
    //        city,
    //    }
    //}
}

impl Table for User {
    fn store(&mut self, driver: PgDriver) {
        let cols = Vec::from(User::FIELD_NAMES);
        let vals = self.vals();
        let id = Self::insert(
            driver,
            "users",
            cols,
            vals,
        );
        self.id = Some(id);
    }

    fn retrieve(driver: PgDriver) -> Vec<User> {
        let cols = vec![String::from("*")];
        let condition = None;
        let res = Self::read(
            driver,
            "users",
            cols,
            condition,
        );
        todo!("Convert the result into a Vec<User>")
    }

    fn update(&self, driver: PgDriver) {
        let cols = Vec::from(User::FIELD_NAMES);
        let vals = self.vals();
        let condition = format!("userid={}", self.id.unwrap());
        Self::alter(
            driver,
            "users",
            cols,
            vals,
            Some(&condition),
        );
    }

    fn remove(&self, driver: PgDriver) {
        Self::delete(driver, "users", self.id.unwrap());
    }
}

#[test]
pub fn test_user_insertion() {
    let mut user = User::new(
        String::from("Max"),
        String::from("Mustermann"),
        String::from("Musterstr. 1"),
        String::from("Mustercity"),
    );
    let mut driver = PgDriver::setup().unwrap();
    driver.connect().unwrap();
    user.store(driver);
}

#[test]
pub fn test_user_retrieval() {
    let mut driver = PgDriver::setup().unwrap();
    driver.connect().unwrap();
    User::retrieve(driver);
}

#[ignore]
#[test]
pub fn test_user_update() {
    let mut driver = PgDriver::setup().unwrap();
    driver.connect().unwrap();
    let user = User::new_with_id( // User::new_with_id isn't available for safety reasons
                                  1,
                                  String::from("Rainer"),
                                  String::from("Zufall"),
                                  String::from("Zufallstr. 10"),
                                  String::from("Zufallstadt"),
    );
    user.update(driver);
}

#[test]
pub fn test_user_deletion() {
    let mut driver = PgDriver::setup().unwrap();
    driver.connect().unwrap();
    let user = User::new_with_id( // User::new_with_id isn't available for safety reasons
                                  1,
                                  String::from("Rainer"),
                                  String::from("Zufall"),
                                  String::from("Zufallstr. 10"),
                                  String::from("Zufallstadt"),
    );
    user.remove(driver);
}
