use crate::db::db_actions::{DbActions, Table};
use crate::db::model::component::{Component, ComponentType};
use customs::bench_message;
use pg_driver::PgDriver;

pub struct ComponentRepository;

impl ComponentRepository {
    pub fn retrieve_single(driver: &mut PgDriver, condition: Option<String>) -> Component {
        Self::retrieve(driver, condition).first().cloned().unwrap()
    }
}

impl Table<Component> for ComponentRepository {
    fn get_name() -> String {
        String::from("components")
    }

    fn get_fk_uuid_name() -> String {
        String::from("component_uuid")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, c_type")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("c_type")
    }

    fn get_fmt_vals(component: &Component) -> String {
        format!("'{}', '{:?}'", component.uuid.to_string(), component.c_type)
    }

    fn get_fmt_vals_no_id(component: &Component) -> String {
        format!("'{:?}'", component.c_type)
    }
}

impl DbActions<Component, Self> for ComponentRepository {
    #[bench_message("Storing component")]
    fn store(driver: &mut PgDriver, component: &Component) -> anyhow::Result<()> {
        Self::insert(driver, component)
    }

    #[bench_message("Updating component")]
    fn update(driver: &mut PgDriver, component: &Component) -> anyhow::Result<()> {
        Self::alter(driver, &component, component.uuid)
    }

    #[bench_message("Deleting component")]
    fn remove(driver: &mut PgDriver, component: &Component) -> anyhow::Result<()> {
        Self::delete(driver, component.uuid)
    }

    #[bench_message("Retrieving components")]
    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<Component> {
        let mut matches: Vec<Component> = vec![];

        let rows = Self::read(driver, &Self::get_name(), condition);

        for row in rows {
            let c_type = ComponentType::parse(row.get("c_type"));
            matches.push(Component::from(row.get("uuid"), c_type))
        }

        matches
    }
}
