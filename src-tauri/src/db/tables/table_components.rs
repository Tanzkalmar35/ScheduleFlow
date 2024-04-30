use uuid::Uuid;

use crate::db_actions::{DbActions, Table};
use crate::pg_driver::PgDriver;

#[derive(Debug)]
pub enum ComponentType {
    EVENT,
    TODO,
    VENUE,
    OTHER
}

impl ComponentType {
    pub fn parse(c_type: &str) -> Self {
        match c_type {
            "event" => Self::EVENT,
            "todo" => Self::TODO,
            "venue" => Self::VENUE,
            _ => Self::OTHER
        }
    }
}

pub struct IComponent {
    uuid: Uuid,
    c_type: ComponentType,
}

impl IComponent {
    pub fn new(c_type: ComponentType) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            c_type,
        }
    }

    pub fn from(uuid: Uuid, c_type: ComponentType) -> Self {
        Self {
            uuid,
            c_type,
        }
    }
}

impl Table for &IComponent {
    fn get_name() -> String {
        String::from("components")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, c_type")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("c_type")
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}', '{:?}'", self.uuid.to_string(), self.c_type)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        format!("'{:?}'", self.c_type)
    }
}

impl Table for IComponent {
    fn get_name() -> String {
        String::from("components")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, c_type")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("c_type")
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}', '{:?}'", self.uuid.to_string(), self.c_type)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        format!("'{:?}'", self.c_type)
    }
}

impl DbActions for IComponent {
    type Item = IComponent;

    fn store(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::insert(driver, self)
    }

    fn update(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::alter(driver, self, self.uuid)
    }

    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::delete::<IComponent>(driver, self.uuid)
    }

    fn retrieve(driver: &mut PgDriver, mut cols: Vec<String>, condition: Option<String>) -> Vec<Self::Item> {
        let mut matches: Vec<IComponent> = vec![];

        if cols.contains(&"*".to_string()) && cols.len() == 1 {
            cols = IComponent::get_fmt_cols().split(", ").map(|c| c.to_string()).collect();
        }

        if let Ok(res) = Self::read(driver, Self::get_name().as_str(), cols, condition) {
            for entry in res {
                let c_type = ComponentType::parse(entry.get("c_type"));
                matches.push(IComponent::from(entry.get("uuid"), c_type))
            }
        };

        matches
    }
}

#[cfg(test)]
mod tests {
    use crate::db_actions::DbActions;
    use crate::pg_driver::PgDriver;

    use super::*;

    #[test]
    fn test_insert() {
        let mut driver = PgDriver::setup();
        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        let component = IComponent::new(ComponentType::EVENT);
        assert!(component.store(&mut driver).is_ok());
    }

    #[test]
    fn test_update() {
        let mut driver = PgDriver::setup();
        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        let mut component = IComponent::new(ComponentType::EVENT);
        assert!(component.store(&mut driver).is_ok());
        component.c_type = ComponentType::TODO;
        assert!(component.update(&mut driver).is_ok());
    }

    #[test]
    fn test_delete() {
        let mut driver = PgDriver::setup();
        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        let component = IComponent::new(ComponentType::EVENT);
        assert!(component.store(&mut driver).is_ok());
        assert!(component.remove(&mut driver).is_ok());
    }

    #[test]
    fn test_retrieve() {
        let mut driver = PgDriver::setup();
        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        let component = IComponent::new(ComponentType::EVENT);
        assert!(component.store(&mut driver).is_ok());
        let retrieved = IComponent::retrieve(&mut driver, vec![String::from("*")], None);
        assert!(retrieved.len() >= 1);
    }
}
