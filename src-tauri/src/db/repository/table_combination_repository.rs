use crate::db::model::calendar::Calendar;
use crate::db::model::component::Component;
use crate::db::repository::calendar_repository::CalendarRepository;
use crate::db::repository::component_repository::ComponentRepository;
use crate::db::{
    db_actions::{DbActions, Table},
    model::{model::Model, table_combination::TableCombination},
    pg_driver::PgDriver,
};
use postgres::Row;
use crate::db::model::jwt_token::JwtToken;
use crate::db::model::property::Property;
use crate::db::model::user::User;
use crate::db::repository::jwt_token_repository::JwtTokenRepository;
use crate::db::repository::property_repository::PropertyRepository;
use crate::db::repository::user_repository::UserRepository;

type CalendarComponentCombination =
    TableCombination<CalendarRepository, ComponentRepository, Calendar, Component>;
type CalendarPropertiesCombination =
    TableCombination<CalendarRepository, PropertyRepository, Calendar, Property>;
type ComponentsPropertiesCombination =
TableCombination<ComponentRepository, PropertyRepository, Component, Property>;
type UserJwtTokenCombination =
TableCombination<UserRepository, JwtTokenRepository, User, JwtToken>;

pub(crate) struct TableCombinationRepository;

impl TableCombinationRepository {}

impl Table<CalendarComponentCombination> for TableCombinationRepository {
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

    fn get_fmt_vals(
        model: &CalendarComponentCombination,
    ) -> String {
        format!("'{}', '{}'", model.uuid1, model.uuid2)
    }

    fn get_fmt_vals_no_id(
        model: &CalendarComponentCombination,
    ) -> String {
        "".to_string()
    }
}

impl DbActions<CalendarComponentCombination, TableCombinationRepository>
    for TableCombinationRepository
{
    fn store(
        driver: &mut PgDriver,
        model: &CalendarComponentCombination,
    ) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(
        _driver: &mut PgDriver,
        _model: &CalendarComponentCombination,
    ) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(
        driver: &mut PgDriver,
        model: &CalendarComponentCombination,
    ) -> anyhow::Result<()> {
        let col_name: String = CalendarRepository::get_fk_uuid_name();
        let col_value: String = model.uuid1.to_string();
        TableCombinationRepository::delete_spec_col(driver, col_name, col_value)
    }

    fn retrieve(
        driver: &mut PgDriver,
        condition: Option<String>,
    ) -> Vec<TableCombination<CalendarRepository, ComponentRepository, Calendar, Component>> {
        let mut res: Vec<CalendarComponentCombination> = vec![];

        let rows: Vec<Row> = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let uuid1 = row.get::<&str, String>(CalendarRepository::get_fk_uuid_name().as_str());
            let uuid2 = row.get::<&str, String>(ComponentRepository::get_fk_uuid_name().as_str());
            res.push(TableCombination::new(
                uuid1.parse().unwrap(),
                uuid2.parse().unwrap(),
            ))
        }

        res
    }
}

impl Table<CalendarPropertiesCombination> for TableCombinationRepository {
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

    fn get_fmt_vals(
        model: &CalendarPropertiesCombination,
    ) -> String {
        format!("'{}', '{}'", model.uuid1, model.uuid2)
    }

    fn get_fmt_vals_no_id(
        model: &CalendarPropertiesCombination,
    ) -> String {
        "".to_string()
    }
}

impl DbActions<CalendarPropertiesCombination, TableCombinationRepository>
for TableCombinationRepository
{
    fn store(
        driver: &mut PgDriver,
        model: &CalendarPropertiesCombination,
    ) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(
        _driver: &mut PgDriver,
        _model: &CalendarPropertiesCombination,
    ) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(
        driver: &mut PgDriver,
        model: &CalendarPropertiesCombination,
    ) -> anyhow::Result<()> {
        let col_name: String = CalendarRepository::get_fk_uuid_name();
        let col_value: String = model.uuid1.to_string();
        TableCombinationRepository::delete_spec_col(driver, col_name, col_value)
    }

    fn retrieve(
        driver: &mut PgDriver,
        condition: Option<String>,
    ) -> Vec<CalendarPropertiesCombination> {
        let mut res: Vec<CalendarPropertiesCombination> = vec![];

        let rows: Vec<Row> = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let uuid1 = row.get::<&str, String>(CalendarRepository::get_fk_uuid_name().as_str());
            let uuid2 = row.get::<&str, String>(PropertyRepository::get_fk_uuid_name().as_str());
            res.push(TableCombination::new(
                uuid1.parse().unwrap(),
                uuid2.parse().unwrap(),
            ))
        }

        res
    }
}

impl Table<ComponentsPropertiesCombination> for TableCombinationRepository {
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

    fn get_fmt_vals(
        model: &ComponentsPropertiesCombination,
    ) -> String {
        format!("'{}', '{}'", model.uuid1, model.uuid2)
    }

    fn get_fmt_vals_no_id(
        model: &ComponentsPropertiesCombination,
    ) -> String {
        "".to_string()
    }
}

impl DbActions<ComponentsPropertiesCombination, TableCombinationRepository>
for TableCombinationRepository
{
    fn store(
        driver: &mut PgDriver,
        model: &ComponentsPropertiesCombination,
    ) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(
        _driver: &mut PgDriver,
        _model: &ComponentsPropertiesCombination,
    ) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(
        driver: &mut PgDriver,
        model: &ComponentsPropertiesCombination,
    ) -> anyhow::Result<()> {
        let col_name: String = ComponentRepository::get_fk_uuid_name();
        let col_value: String = model.uuid1.to_string();
        TableCombinationRepository::delete_spec_col(driver, col_name, col_value)
    }

    fn retrieve(
        driver: &mut PgDriver,
        condition: Option<String>,
    ) -> Vec<ComponentsPropertiesCombination> {
        let mut res: Vec<ComponentsPropertiesCombination> = vec![];

        let rows: Vec<Row> = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let uuid1 = row.get::<&str, String>(ComponentRepository::get_fk_uuid_name().as_str());
            let uuid2 = row.get::<&str, String>(PropertyRepository::get_fk_uuid_name().as_str());
            res.push(TableCombination::new(
                uuid1.parse().unwrap(),
                uuid2.parse().unwrap(),
            ))
        }

        res
    }
}

impl Table<UserJwtTokenCombination> for TableCombinationRepository {
    fn get_name() -> String {
        format!(
            "{}_{}",
            UserRepository::get_name(),
            JwtTokenRepository::get_name()
        )
    }

    fn get_fk_uuid_name() -> String {
        "".to_string()
    }

    fn get_fmt_cols() -> String {
        format!(
            "{}, {}",
            UserRepository::get_fk_uuid_name(),
            JwtTokenRepository::get_fk_uuid_name()
        )
    }

    fn get_fmt_cols_no_id() -> String {
        "".to_string()
    }

    fn get_fmt_vals(
        model: &UserJwtTokenCombination,
    ) -> String {
        format!("'{}', '{}'", model.uuid1, model.uuid2)
    }

    fn get_fmt_vals_no_id(
        model: &UserJwtTokenCombination,
    ) -> String {
        "".to_string()
    }
}

impl DbActions<UserJwtTokenCombination, TableCombinationRepository>
for TableCombinationRepository
{
    fn store(
        driver: &mut PgDriver,
        model: &UserJwtTokenCombination,
    ) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(
        _driver: &mut PgDriver,
        _model: &UserJwtTokenCombination,
    ) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(
        driver: &mut PgDriver,
        model: &UserJwtTokenCombination,
    ) -> anyhow::Result<()> {
        let col_name: String = UserRepository::get_fk_uuid_name();
        let col_value: String = model.uuid1.to_string();
        TableCombinationRepository::delete_spec_col(driver, col_name, col_value)
    }

    fn retrieve(
        driver: &mut PgDriver,
        condition: Option<String>,
    ) -> Vec<UserJwtTokenCombination> {
        let mut res: Vec<UserJwtTokenCombination> = vec![];

        let rows: Vec<Row> = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let uuid1 = row.get::<&str, String>(UserRepository::get_fk_uuid_name().as_str());
            let uuid2 = row.get::<&str, String>(JwtTokenRepository::get_fk_uuid_name().as_str());
            res.push(TableCombination::new(
                uuid1.parse().unwrap(),
                uuid2.parse().unwrap(),
            ))
        }

        res
    }
}