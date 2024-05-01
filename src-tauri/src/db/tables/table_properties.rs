use uuid::Uuid;

use crate::db_actions::{DbActions, Table};
use crate::pg_driver::PgDriver;

struct IProperty {
    uuid: Uuid,
    key: String,
    val: String,
}

impl IProperty {
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
}

impl Table for IProperty {
    fn get_name() -> String {
        String::from("properties")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, key, value")
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

impl Table for &IProperty {
    fn get_name() -> String {
        String::from("properties")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, key, value")
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

impl DbActions for IProperty {
    type Item = IProperty;

    fn store(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::insert(driver, self)
    }

    fn update(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::alter(driver, self, self.uuid)
    }

    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::delete::<IProperty>(driver, self.uuid)
    }

    fn retrieve(driver: &mut PgDriver, mut cols: Vec<String>, condition: Option<String>) -> Vec<Self::Item> {
        let mut matches: Vec<IProperty> = vec![];

        if cols.contains(&"*".to_string()) && cols.len() == 1 {
            cols = IProperty::get_fmt_cols().split(", ").map(|c| c.to_string()).collect();
        }

        if let Ok(rows) = Self::read(driver, Self::get_name().as_str(), cols, condition) {
            for row in rows {
                matches.push(IProperty::from(
                    row.get("uuid"),
                    row.get("key"),
                    row.get("value")
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
        let property = IProperty::new(String::from("test_key"), String::from("test_value"));
        assert!(property.store(&mut driver).is_ok());
    }

    #[test]
    fn test_update() {
        let mut driver = PgDriver::setup();
        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        let mut property = IProperty::new(String::from("test_key"), String::from("test_value"));
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
        let property = IProperty::new(String::from("test_key"), String::from("test_value"));
        assert!(property.store(&mut driver).is_ok());
        assert!(property.remove(&mut driver).is_ok());
    }

    #[test]
    fn test_retrieve() {
        let mut driver = PgDriver::setup();
        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        let property = IProperty::new(String::from("test_key"), String::from("test_value"));
        assert!(property.store(&mut driver).is_ok());
        let retrieved = IProperty::retrieve(&mut driver, vec![String::from("*")], None);
        assert!(retrieved.len() >= 1);
    }
}
