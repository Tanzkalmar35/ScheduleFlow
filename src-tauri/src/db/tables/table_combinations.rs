use std::marker::PhantomData;
use uuid::Uuid;
use crate::db_actions::{DbActions, Table};
use crate::pg_driver::PgDriver;

pub struct TableCombination<T1: Table, T2: Table> {
    pub(crate) uuid1: Uuid,
    pub(crate) uuid2: Uuid,
    phantom1: PhantomData<T1>,
    phantom2: PhantomData<T2>,
}

impl<T1: Table, T2: Table> TableCombination<T1, T2> {
    pub fn new(uuid1: Uuid, uuid2: Uuid) -> Self {
        Self {
            uuid1,
            uuid2,
            phantom1: Default::default(),
            phantom2: Default::default(),
        }
    }
}

impl<T1: Table, T2: Table> Table for &TableCombination<T1, T2> {
    fn get_name() -> String {
        format!("{}_{}", T1::get_name(), T2::get_name())
    }

    fn get_fmt_cols() -> String {
        format!("{}, {}", T1::get_fk_uuid_name(), T2::get_fk_uuid_name())
    }

    fn get_fk_uuid_name() -> String {
        unimplemented!("A table combination can't be combined.")
    }

    fn get_fmt_cols_no_id() -> String {
        unimplemented!("A combination consists of uuids.")
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}', '{}'", self.uuid1, self.uuid2)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        unimplemented!("A combination consists of uuids.")
    }
}

impl<T1: Table, T2: Table> Table for TableCombination<T1, T2> {
    fn get_name() -> String {
        format!("{}_{}", T1::get_name(), T2::get_name())
    }

    fn get_fmt_cols() -> String {
        format!("{}, {}", T1::get_fk_uuid_name(), T2::get_fk_uuid_name())
    }

    fn get_fk_uuid_name() -> String {
        unimplemented!("A table combination can't be combined.")
    }

    fn get_fmt_cols_no_id() -> String {
        unimplemented!("A combination consists of uuids.")
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}', '{}'", self.uuid1, self.uuid2)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        unimplemented!("A combination consists of uuids.")
    }
}

impl<T1: Table, T2: Table> DbActions for TableCombination<T1, T2> {
    type Item = Self;

    fn store(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::insert(driver, self)
    }

    fn update(&self, _driver: &mut PgDriver) -> anyhow::Result<()> {
        unimplemented!("You can't update a combination entry, as it does only consist of two uuids.")
    }

    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::delete_spec_col::<&Self>(driver, T1::get_fk_uuid_name(), self.uuid1)
    }

    fn retrieve(driver: &mut PgDriver, mut cols: Vec<String>, condition: Option<String>) -> Vec<Self::Item> {
        let mut res: Vec<Self> = vec![];

        if cols.contains(&"*".to_string()) && cols.len() == 1 {
            cols = Self::get_fmt_cols().split(", ").map(|c| c.to_string()).collect();
        }
        if let Ok(rows) = Self::read(driver, Self::get_name().as_str(), cols, condition) {
            for row in rows {
                let uuid1 = row.get(T1::get_fk_uuid_name().as_str());
                let uuid2 = row.get(T2::get_fk_uuid_name().as_str());
                res.push(Self::new(uuid1, uuid2))
            };
        }

        res
    }
}

#[cfg(test)]
mod tests_icalendar_icomponent {
    use crate::db_actions::DbActions;
    use crate::pg_driver::PgDriver;
    use crate::table_calendars::CalendarDAO;
    use crate::table_combinations::TableCombination;
    use crate::table_components::{ComponentType, ComponentDAO};

    #[test]
    pub fn test_storing_calendar_component_combination() {
        let mut driver = PgDriver::setup();
        let cal = CalendarDAO::new();
        let component = ComponentDAO::new(ComponentType::EVENT);

        if let Err(e) = driver.connect() {
            panic!("Driver conn failed: {}", e)
        }
        if let Err(e) = cal.store(&mut driver) {
            panic!("Storing calendar failed: {}", e)
        }
        if let Err(e) = component.store(&mut driver) {
            panic!("Storing event calendar failed: {}", e)
        }
        let calendar_component: TableCombination<CalendarDAO, ComponentDAO> = TableCombination::new(cal.uuid, component.uuid);
        if let Err(e) = calendar_component.store(&mut driver) {
            panic!("Storing combination failed: {}", e)
        }
    }

    #[test]
    pub fn test_removing_calendar_component_combination() {
        let mut driver = PgDriver::setup();
        let cal = CalendarDAO::new();
        let component = ComponentDAO::new(ComponentType::EVENT);

        if let Err(e) = driver.connect() {
            panic!("Driver conn failed: {}", e)
        }
        if let Err(e) = cal.store(&mut driver) {
            panic!("Storing calendar failed: {}", e)
        }
        if let Err(e) = component.store(&mut driver) {
            panic!("Storing event calendar failed: {}", e)
        }
        let calendar_component: TableCombination<CalendarDAO, ComponentDAO> = TableCombination::new(cal.uuid, component.uuid);
        if let Err(e) = calendar_component.store(&mut driver) {
            panic!("Storing combination failed: {}", e)
        }
        if let Err(e) = calendar_component.remove(&mut driver) {
            panic!("Removing combination failed: {}", e)
        }
    }

    #[test]
    pub fn test_retrieving_calendar_component_combination() {
        let mut driver = PgDriver::setup();
        let cal = CalendarDAO::new();
        let component = ComponentDAO::new(ComponentType::EVENT);

        if let Err(e) = driver.connect() {
            panic!("Driver conn failed: {}", e)
        }
        if let Err(e) = cal.store(&mut driver) {
            panic!("Storing calendar failed: {}", e)
        }
        if let Err(e) = component.store(&mut driver) {
            panic!("Storing event calendar failed: {}", e)
        }
        let calendar_component: TableCombination<CalendarDAO, ComponentDAO> = TableCombination::new(cal.uuid, component.uuid);
        if let Err(e) = calendar_component.store(&mut driver) {
            panic!("Storing combination failed: {}", e)
        }
        assert!(TableCombination::<CalendarDAO, ComponentDAO>::retrieve(&mut driver, vec!["*".to_string()], None).len() >= 1)
    }
}

#[cfg(test)]
mod tests_icalendar_iproperty {
    use crate::db_actions::DbActions;
    use crate::pg_driver::PgDriver;
    use crate::table_calendars::CalendarDAO;
    use crate::table_combinations::TableCombination;
    use crate::table_properties::PropertyDAO;

    #[test]
    pub fn test_storing_calendar_property_combination() {
        let mut driver = PgDriver::setup();
        let cal = CalendarDAO::new();
        let property = PropertyDAO::new(String::from("test_key"), String::from("test_value"));
        let combination = TableCombination::<CalendarDAO, PropertyDAO>::new(cal.uuid, property.uuid);

        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        if let Err(e) = cal.store(&mut driver) {
            panic!("Error storing calendar: {}", e)
        }
        if let Err(e) = property.store(&mut driver) {
            panic!("Error storing property: {}", e)
        }
        if let Err(e) = combination.store(&mut driver) {
            panic!("Error storing combination: {}", e)
        }
    }

    #[test]
    pub fn test_removing_calendar_property_combination() {
        let mut driver = PgDriver::setup();
        let cal = CalendarDAO::new();
        let property = PropertyDAO::new(String::from("test_key"), String::from("test_value"));
        let combination = TableCombination::<CalendarDAO, PropertyDAO>::new(cal.uuid, property.uuid);

        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        if let Err(e) = cal.store(&mut driver) {
            panic!("Error storing calendar: {}", e)
        }
        if let Err(e) = property.store(&mut driver) {
            panic!("Error storing property: {}", e)
        }
        if let Err(e) = combination.store(&mut driver) {
            panic!("Error storing combination: {}", e)
        }
        if let Err(e) = combination.remove(&mut driver) {
            panic!("Error deleting combination: {}", e)
        }
    }

    #[test]
    pub fn test_retrieving_calendar_property_combination() {
        let mut driver = PgDriver::setup();
        let cal = CalendarDAO::new();
        let property = PropertyDAO::new(String::from("test_key"), String::from("test_value"));
        let combination = TableCombination::<CalendarDAO, PropertyDAO>::new(cal.uuid, property.uuid);

        if let Err(_) = driver.connect() {
            panic!("Driver conn failed")
        }
        if let Err(e) = cal.store(&mut driver) {
            panic!("Error storing calendar: {}", e)
        }
        if let Err(e) = property.store(&mut driver) {
            panic!("Error storing property: {}", e)
        }
        if let Err(e) = combination.store(&mut driver) {
            panic!("Error storing combination: {}", e)
        }
        assert!(TableCombination::<CalendarDAO, PropertyDAO>::retrieve(&mut driver, vec!["*".to_string()], None).len() >= 1)
    }
}

