use uuid::Uuid;
use crate::db_actions::Table;
use crate::pg_driver::PgDriver;

pub struct ICalendar {
    id: String,
    name: String,
}

impl ICalendar {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
        }
    }

    pub fn field_names() -> Vec<&'static str> {
        vec!["id", "name"]
    }
}

impl Table for ICalendar {
    fn store(&mut self, driver: &mut PgDriver) -> anyhow::Result<()> {
        let vals = vec![self.id.as_str(), self.name.as_str()];
        Self::insert(driver, "calendars", Self::field_names(), vals)
    }

    fn update(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        let cols = vec!["name"];
        let vals = vec![self.name.as_str()];
        Self::alter(driver, "calendars", cols, vals, self.id.clone())
    }

    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use crate::db_actions::Table;
    use crate::pg_driver::PgDriver;
    use crate::table_calendars::ICalendar;

    #[test]
    pub fn test_calendar_insertion() {
        let mut res = false;
        let mut cal = ICalendar::new(String::from("SOME CALENDAR"));
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

    #[test]
    pub fn test_calendar_update() {
        let mut res = false;
        let mut cal = ICalendar::new(String::from("SOME CALENDAR"));
        let start_time;
        let mut elapsed_time = Duration::from_millis(0);

        let mut driver = PgDriver::setup();
        match driver.connect() {
            Ok(driver) => {
                cal.store(driver).expect("Storing calendar failed at calendar.store()");
                cal.name = String::from("SOME OTHER CALENDAR");
                cal.update(driver).expect("Calendar update failed");
                start_time = Instant::now();
                elapsed_time = start_time.elapsed();
                res = true;
            },
            Err(e) => println!("{}", e)
        }

        println!("Inserting a new empty calendar (only a name) took: {:?}", elapsed_time);
        assert_eq!(res, true);
    }
}
