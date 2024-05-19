use uuid::Uuid;

use crate::db_actions::{DbActions, Table};
use crate::pg_driver::PgDriver;

#[derive(Debug)]
pub struct CalendarDAO {
    pub(crate) uuid: Uuid,
}

impl CalendarDAO {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}

impl Table for CalendarDAO {
    fn get_name() -> String {
        String::from("calendars")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid")
    }

    fn get_fk_uuid_name() -> String {
        String::from("calendar_uuid")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("")
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}'", self.uuid)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        format!("")
    }
}

impl Table for &CalendarDAO {
    fn get_name() -> String {
        String::from("calendars")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid")
    }

    fn get_fk_uuid_name() -> String {
        String::from("calendar_uuid")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("")
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}'", self.uuid)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        format!("")
    }
}

impl DbActions for CalendarDAO {
    type Item = CalendarDAO;

    fn store(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::insert(driver, self)
    }

    /// As a calendar db entry only has an uuid, it should have no reason to change.
    fn update(&self, _driver: &mut PgDriver) -> anyhow::Result<()> {
        unimplemented!("You can't update a calendar db entry, as it does only consist of an uuid.")
    }

    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::delete::<&Self>(driver, self.uuid)
    }

    fn retrieve(driver: &mut PgDriver, mut cols: Vec<String>, condition: Option<String>) -> Vec<Self::Item> {
        let mut res: Vec<CalendarDAO> = vec![];

        if cols.contains(&"*".to_string()) && cols.len() == 1 {
            cols = CalendarDAO::get_fmt_cols().split(", ").map(|c| c.to_string()).collect();
        }
        if let Ok(rows) = Self::read(driver, "calendars", cols, condition) {
            for row in rows {
                let val = row.get("uuid");
                res.push(CalendarDAO { uuid: val })
            };
        }

        res
    }
}

#[cfg(test)]
mod tests {

    use crate::db_actions::DbActions;
    use crate::pg_driver::PgDriver;
    use crate::table_calendars::CalendarDAO;

    #[test]
    pub fn test_calendar_insertion() {
        let mut res = false;
        let cal = CalendarDAO::new();

        let mut driver = PgDriver::setup();
        match driver.connect() {
            Ok(driver) => {
                cal.store(driver).expect("Storing calendar failed at calendar.store()");
                res = true;
            }
            Err(e) => println!("{}", e)
        }

        assert_eq!(res, true);
    }

    #[test]
    pub fn test_calendar_deletion() {
        let mut res = false;
        let cal = CalendarDAO::new();
        let mut driver = PgDriver::setup();

        match driver.connect() {
            Ok(driver) => {
                cal.store(driver).expect("Storing calendar failed at calendar.store()");
                match cal.remove(driver) {
                    Ok(_) => res = true,
                    Err(e) => println!("Removing calendar failed with error: {}", e)
                }
            }
            Err(e) => println!("{}", e)
        }

        assert_eq!(res, true);
    }

    #[test]
    fn test_retrieve_all_columns_no_condition() {
        let mut driver = PgDriver::setup();
        driver.connect().unwrap();

        // Store enough calendars
        for _ in 0..10 {
            let cal = CalendarDAO::new();
            cal.store(&mut driver).unwrap();
        }

        let result = CalendarDAO::retrieve(&mut driver, vec!["*".to_string()], None);
        assert!(result.len() >= 10); // Assert that at least 10 calendars were retrieved
    }

    #[test]
    fn test_retrieve_one_column_no_condition() {
        let mut driver = PgDriver::setup();
        driver.connect().unwrap();

        // Store enough calendars
        for _ in 0..10 {
            let cal = CalendarDAO::new();
            cal.store(&mut driver).unwrap();
        }

        let result = CalendarDAO::retrieve(&mut driver, vec!["uuid".to_string()], None);
        assert!(result.len() >= 10); // Assert that at least 10 calendars were retrieved
    }

    #[test]
    fn test_retrieve_with_condition() {
        let mut driver = PgDriver::setup();
        driver.connect().unwrap();

        // Store a calendar and keep its uuid
        let cal = CalendarDAO::new();
        cal.store(&mut driver).unwrap();
        let uuid = cal.uuid;

        let condition = format!("uuid = '{}'", uuid);
        let result = CalendarDAO::retrieve(&mut driver, vec!["*".to_string()], Some(condition));
        assert_eq!(result.len(), 1); // Assert that only one calendar was retrieved
        assert_eq!(result[0].uuid, uuid); // Assert that the retrieved calendar has the correct uuid
    }
}
