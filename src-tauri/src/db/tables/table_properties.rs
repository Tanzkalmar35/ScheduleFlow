use std::iter::Cloned;
use std::ops::Deref;
use uuid::Uuid;

use crate::db_actions::{DbActions, Table};
use crate::pg_driver::PgDriver;

#[derive(Clone)]
pub struct PropertyDAO {
    pub(crate) uuid: Uuid,
    pub(crate) key: String,
    pub(crate) val: String,
}

impl PropertyDAO {
    pub fn new(key: String, val: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            key,
            val,
        }
    }

    pub fn from(uuid: Uuid, key: String, val: String) -> Self {
        Self {
            uuid,
            key,
            val,
        }
    }

    pub fn retrieve_single(driver: &mut PgDriver, condition: Option<String>) -> Self {
        Self::retrieve(driver, condition).first().cloned().unwrap()
    }
}

impl Table for PropertyDAO {
    fn get_name() -> String {
        String::from("properties")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, key, value")
    }

    fn get_fk_uuid_name() -> String {
        String::from("property_uuid")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("key, value")
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}', '{}', '{}'", self.uuid, self.key, self.val)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        format!("'{}', '{}'", self.key, self.val)
    }
}

impl Table for &PropertyDAO {
    fn get_name() -> String {
        String::from("properties")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, key, value")
    }

    fn get_fk_uuid_name() -> String {
        String::from("property_uuid")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("key, value")
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}', '{}', '{}'", self.uuid, self.key, self.val)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        format!("'{}', '{}'", self.key, self.val)
    }
}

impl DbActions for PropertyDAO {
    type Item = PropertyDAO;

    fn store(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::insert(driver, self)
    }

    fn update(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::alter(driver, self, self.uuid)
    }

    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::delete::<PropertyDAO>(driver, self.uuid)
    }

    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<Self::Item> {
        let mut matches: Vec<PropertyDAO> = vec![];

        if let Ok(rows) = Self::read(driver, Self::get_name().as_str(), condition) {
            for row in rows {
                matches.push(PropertyDAO::from(
                    row.get("uuid"),
                    row.get("key"),
                    row.get("value"),
                ));
            }
        }

        matches
    }
}

#[cfg(test)]
mod tests {
    use crate::db_actions::DbActions;
    use crate::pg_driver::PgDriver;

    use super::*;

    #[test]
    fn test_store() {
        let mut driver = PgDriver::setup();
        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        let property = PropertyDAO::new(String::from("test_key"), String::from("test_value"));
        assert!(property.store(&mut driver).is_ok());
    }

    #[test]
    fn test_update() {
        let mut driver = PgDriver::setup();
        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        let mut property = PropertyDAO::new(String::from("test_key"), String::from("test_value"));
        assert!(property.store(&mut driver).is_ok());
        property.key = String::from("updated_key");
        property.val = String::from("updated_value");
        assert!(property.update(&mut driver).is_ok());
    }

    #[test]
    fn test_remove() {
        let mut driver = PgDriver::setup();
        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        let property = PropertyDAO::new(String::from("test_key"), String::from("test_value"));
        assert!(property.store(&mut driver).is_ok());
        assert!(property.remove(&mut driver).is_ok());
    }

    #[test]
    fn test_retrieve() {
        let mut driver = PgDriver::setup();
        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        let property = PropertyDAO::new(String::from("test_key"), String::from("test_value"));
        assert!(property.store(&mut driver).is_ok());
        let retrieved = PropertyDAO::retrieve(&mut driver, None);
        assert!(retrieved.len() >= 1);
    }
}
