use icalendar::Calendar;
use crate::db_actions::Table;
use crate::pg_driver::PgDriver;

impl Table for Calendar {
    fn store(&mut self, driver: &mut PgDriver) -> anyhow::Result<()> {
        todo!()
    }

    fn update(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        todo!()
    }

    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        todo!()
    }
}
