use crate::db::{
    db_actions::{DbActions, Table},
    model::calendar::Calendar,
    pg_driver::PgDriver,
};

pub struct CalendarRepository;

impl CalendarRepository {}

impl Table<Calendar> for CalendarRepository {
    fn get_name() -> String {
        String::from("calendars")
    }

    fn get_fk_uuid_name() -> String {
        String::from("calendar_uuid")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("")
    }

    fn get_fmt_vals(calendar: &Calendar) -> String {
        format!("'{}'", calendar.uuid)
    }

    fn get_fmt_vals_no_id(calendar: &Calendar) -> String {
        "".to_string()
    }
}

impl DbActions<Calendar, Self> for CalendarRepository {
    fn store(driver: &mut PgDriver, calendar: &Calendar) -> anyhow::Result<()> {
        Self::insert(driver, calendar)
    }

    /// As a calendar db entry only has an uuid, it should have no reason to change.
    fn update(_driver: &mut PgDriver, model: &Calendar) -> anyhow::Result<()> {
        unimplemented!("You can't update a calendar db entry, as it does only consist of an uuid.")
    }

    fn remove(driver: &mut PgDriver, calendar: &Calendar) -> anyhow::Result<()> {
        Self::delete(driver, calendar.uuid)
    }

    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<Calendar> {
        let mut res: Vec<Calendar> = vec![];

        let rows = Self::read(driver, &Self::get_name(), condition);

        for row in rows {
            let uuid = row.get("uuid");
            res.push(Calendar::from(uuid))
        }

        res
    }
}
