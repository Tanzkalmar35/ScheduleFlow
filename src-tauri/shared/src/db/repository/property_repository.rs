use crate::db::db_actions::{DbActions, Table};
use crate::db::model::property::Property;
use customs::bench_message;
use pg_driver::PgDriver;

pub struct PropertyRepository;

impl PropertyRepository {
    pub fn retrieve_first(driver: &mut PgDriver, condition: Option<String>) -> Option<Property> {
        let mut res;

        if let Some(condition) = condition {
            let limit_condition = format!("{} {}", condition, "LIMIT 1");
            res = PropertyRepository::retrieve(driver, Some(limit_condition));
        } else {
            res = PropertyRepository::retrieve(driver, condition);
        }

        if res.len() >= 1 {
            Some(res.remove(0))
        } else {
            None
        }
    }
}

impl Table<Property> for PropertyRepository {
    fn get_name() -> String {
        String::from("properties")
    }

    fn get_fk_uuid_name() -> String {
        String::from("property_uuid")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, key, value")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("key, value")
    }

    fn get_fmt_vals(property: &Property) -> String {
        format!(
            "'{}', '{}', '{}'",
            property.get_uuid(),
            property.get_key(),
            property.get_val()
        )
    }

    fn get_fmt_vals_no_id(property: &Property) -> String {
        format!("'{}', '{}'", property.get_key(), property.get_val())
    }
}

impl DbActions<Property, Self> for PropertyRepository {
    #[bench_message("Storing property")]
    fn store(driver: &mut PgDriver, model: &Property) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    #[bench_message("Updating property")]
    fn update(driver: &mut PgDriver, model: &Property) -> anyhow::Result<()> {
        Self::alter(driver, &model, model.get_uuid())
    }

    #[bench_message("Deleting property")]
    fn remove(driver: &mut PgDriver, model: &Property) -> anyhow::Result<()> {
        Self::delete(driver, model.get_uuid())
    }

    #[bench_message("Retrieving properties")]
    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<Property> {
        let mut matches: Vec<Property> = vec![];

        let rows = Self::read(driver, &Self::get_name(), condition);

        for row in rows {
            matches.push(Property::from(
                row.get("uuid"),
                row.get("key"),
                row.get("value"),
            ));
        }

        matches
    }
}
