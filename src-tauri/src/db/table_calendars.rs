use uuid::Uuid;
use crate::db_actions::{DbActions, Table};
use crate::pg_driver::PgDriver;

#[derive(Debug)]
pub struct ICalendar {
    uuid: Uuid,
}

impl ICalendar {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}

impl Table for &ICalendar {
    fn get_name(&self) -> String {
        String::from("calendars")
    }

    fn get_fmt_cols(&self) -> String {
        String::from("uuid")
    }

    fn get_fmt_cols_no_id(&self) -> String {
        String::from("")
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}'", self.uuid)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        format!("")
    }
}

impl DbActions for ICalendar {
    fn store(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::insert(driver, self)
    }

    /// As a calendar only has an uuid, it should have no reason to change.
    fn update(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        unimplemented!("You can't update a calendar.")
    }

    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::delete(driver, self, self.uuid)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use crate::db_actions::DbActions;
    use crate::pg_driver::PgDriver;
    use crate::table_calendars::ICalendar;

    #[test]
    pub fn test_calendar_insertion() {
        let mut res = false;
        let mut cal = ICalendar::new();
        let start_time;
        let mut elapsed_time = Duration::from_millis(0);

        let mut driver = PgDriver::setup();
        match driver.connect() {
            Ok(driver) => {
                start_time = Instant::now();
                cal.store(driver).expect("Storing calendar failed at calendar.store()");
                elapsed_time = start_time.elapsed();
                res = true;
            },
            Err(e) => println!("{}", e)
        }

        println!("Inserting a new empty calendar (only a name) took: {:?}", elapsed_time);
        assert_eq!(res, true);
    }
}
