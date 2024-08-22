use postgres::Row;

use crate::db::{
    db_actions::{DbActions, Table},
    model::calendar_property_combination::CalendarPropertyCombination,
    pg_driver::PgDriver,
    repository::{
        calendar_repository::CalendarRepository, property_repository::PropertyRepository,
    },
};

pub struct CalendarPropertyCombinationRepository;

impl Table<CalendarPropertyCombination> for CalendarPropertyCombinationRepository {
    fn get_name() -> String {
        format!(
            "{}_{}",
            CalendarRepository::get_name(),
            PropertyRepository::get_name()
        )
    }

    fn get_fk_uuid_name() -> String {
        "".to_string()
    }

    fn get_fmt_cols() -> String {
        format!(
            "{}, {}",
            CalendarRepository::get_fk_uuid_name(),
            PropertyRepository::get_fk_uuid_name()
        )
    }

    fn get_fmt_cols_no_id() -> String {
        "".to_string()
    }

    fn get_fmt_vals(model: &CalendarPropertyCombination) -> String {
        format!("'{}', '{}'", model.calendar_uuid, model.property_uuid)
    }

    fn get_fmt_vals_no_id(model: &CalendarPropertyCombination) -> String {
        "".to_string()
    }
}

impl DbActions<CalendarPropertyCombination, Self> for CalendarPropertyCombinationRepository {
    fn store(driver: &mut PgDriver, model: &CalendarPropertyCombination) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(_driver: &mut PgDriver, _model: &CalendarPropertyCombination) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(driver: &mut PgDriver, model: &CalendarPropertyCombination) -> anyhow::Result<()> {
        let col_name: String = CalendarRepository::get_fk_uuid_name();
        let col_value: String = model.calendar_uuid.to_string();
        Self::delete_spec_col(driver, col_name, col_value)
    }

    fn retrieve(
        driver: &mut PgDriver,
        condition: Option<String>,
    ) -> Vec<CalendarPropertyCombination> {
        let mut res: Vec<CalendarPropertyCombination> = vec![];

        let rows: Vec<Row> = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let calendar_uuid: String = row.get(CalendarRepository::get_fk_uuid_name().as_str());
            let property_uuid: String = row.get(PropertyRepository::get_fk_uuid_name().as_str());
            res.push(CalendarPropertyCombination::new(
                calendar_uuid.parse().unwrap(),
                property_uuid.parse().unwrap(),
            ))
        }

        res
    }
}
