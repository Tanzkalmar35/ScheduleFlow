use postgres::Row;

use crate::db::{
    db_actions::{DbActions, Table},
    model::{
        calendar::Calendar, calendar_component_combination::CalendarComponentCombination,
        component::Component,
    },
    pg_driver::PgDriver,
    repository::{
        calendar_repository::CalendarRepository, component_repository::ComponentRepository,
    },
};

pub struct CalendarComponentCombinationRepository;

impl Table<CalendarComponentCombination> for CalendarComponentCombinationRepository {
    fn get_name() -> String {
        format!(
            "{}_{}",
            CalendarRepository::get_name(),
            ComponentRepository::get_name()
        )
    }

    fn get_fk_uuid_name() -> String {
        "".to_string()
    }

    fn get_fmt_cols() -> String {
        format!(
            "{}, {}",
            CalendarRepository::get_fk_uuid_name(),
            ComponentRepository::get_fk_uuid_name()
        )
    }

    fn get_fmt_cols_no_id() -> String {
        "".to_string()
    }

    fn get_fmt_vals(model: &CalendarComponentCombination) -> String {
        format!("'{}', '{}'", model.calendar_uuid, model.component_uuid)
    }

    fn get_fmt_vals_no_id(model: &CalendarComponentCombination) -> String {
        "".to_string()
    }
}

impl DbActions<CalendarComponentCombination, Self> for CalendarComponentCombinationRepository {
    fn store(driver: &mut PgDriver, model: &CalendarComponentCombination) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(_driver: &mut PgDriver, _model: &CalendarComponentCombination) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(driver: &mut PgDriver, model: &CalendarComponentCombination) -> anyhow::Result<()> {
        let col_name: String = CalendarRepository::get_fk_uuid_name();
        let col_value: String = model.calendar_uuid.to_string();
        Self::delete_spec_col(driver, col_name, col_value)
    }

    fn retrieve(
        driver: &mut PgDriver,
        condition: Option<String>,
    ) -> Vec<CalendarComponentCombination> {
        let mut res: Vec<CalendarComponentCombination> = vec![];

        let rows: Vec<Row> = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let calendar_uuid: String = row.get(CalendarRepository::get_fk_uuid_name().as_str());
            let component_uuid: String = row.get(ComponentRepository::get_fk_uuid_name().as_str());
            res.push(CalendarComponentCombination::new(
                calendar_uuid.parse().unwrap(),
                component_uuid.parse().unwrap(),
            ))
        }

        res
    }
}
