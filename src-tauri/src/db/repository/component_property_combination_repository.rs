use postgres::Row;

use crate::db::{
    db_actions::{DbActions, Table},
    model::component_property_combination::ComponentPropertyCombination,
    pg_driver::PgDriver,
};

use super::{component_repository::ComponentRepository, property_repository::PropertyRepository};

pub struct ComponentPropertyCombinationRepository;

impl Table<ComponentPropertyCombination> for ComponentPropertyCombinationRepository {
    fn get_name() -> String {
        format!(
            "{}_{}",
            ComponentRepository::get_name(),
            PropertyRepository::get_name()
        )
    }

    fn get_fk_uuid_name() -> String {
        "".to_string()
    }

    fn get_fmt_cols() -> String {
        format!(
            "{}, {}",
            ComponentRepository::get_fk_uuid_name(),
            PropertyRepository::get_fk_uuid_name()
        )
    }

    fn get_fmt_cols_no_id() -> String {
        "".to_string()
    }

    fn get_fmt_vals(model: &ComponentPropertyCombination) -> String {
        format!("'{}', '{}'", model.component_uuid, model.property_uuid)
    }

    fn get_fmt_vals_no_id(model: &ComponentPropertyCombination) -> String {
        "".to_string()
    }
}

impl DbActions<ComponentPropertyCombination, Self> for ComponentPropertyCombinationRepository {
    fn store(driver: &mut PgDriver, model: &ComponentPropertyCombination) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(_driver: &mut PgDriver, _model: &ComponentPropertyCombination) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(driver: &mut PgDriver, model: &ComponentPropertyCombination) -> anyhow::Result<()> {
        let col_name: String = ComponentRepository::get_fk_uuid_name();
        let col_value: String = model.component_uuid.to_string();
        Self::delete_spec_col(driver, col_name, col_value)
    }

    fn retrieve(
        driver: &mut PgDriver,
        condition: Option<String>,
    ) -> Vec<ComponentPropertyCombination> {
        let mut res: Vec<ComponentPropertyCombination> = vec![];

        let rows: Vec<Row> = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let calendar_uuid: String = row.get(ComponentRepository::get_fk_uuid_name().as_str());
            let property_uuid: String = row.get(PropertyRepository::get_fk_uuid_name().as_str());
            res.push(ComponentPropertyCombination::new(
                calendar_uuid.parse().unwrap(),
                property_uuid.parse().unwrap(),
            ))
        }

        res
    }
}
