use postgres::Row;
use uuid::Uuid;

use crate::db::{
    db_actions::{DbActions, Table},
    model::user_client_combination::UserClientCombination,
    repository::{client_repository::ClientRepository, user_repository::UserRepository},
};

pub struct UserClientRepository {
    user_uuid: Uuid,
    client_uuid: Uuid,
}

impl Table<UserClientCombination> for UserClientRepository {
    fn get_name() -> String {
        format!(
            "{}_{}",
            UserRepository::get_fk_uuid_name(),
            ClientRepository::get_fk_uuid_name()
        )
    }

    fn get_fk_uuid_name() -> String {
        unimplemented!()
    }

    fn get_fmt_cols() -> String {
        format!(
            "{}, {}",
            UserRepository::get_fk_uuid_name(),
            ClientRepository::get_fk_uuid_name()
        )
    }

    fn get_fmt_cols_no_id() -> String {
        unimplemented!()
    }

    fn get_fmt_vals(model: &UserClientCombination) -> String {
        format!("'{}', '{}'", model.user_uuid, model.client_uuid,)
    }

    fn get_fmt_vals_no_id(model: &UserClientCombination) -> String {
        unimplemented!()
    }
}

impl DbActions<UserClientCombination, Self> for UserClientRepository {
    fn store(
        driver: &mut pg_driver::PgDriver,
        model: &UserClientCombination,
    ) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(
        _driver: &mut pg_driver::PgDriver,
        _model: &UserClientCombination,
    ) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(
        driver: &mut pg_driver::PgDriver,
        model: &UserClientCombination,
    ) -> anyhow::Result<()> {
        let col_name: String = UserRepository::get_fk_uuid_name();
        let col_value: String = model.user_uuid.to_string();
        Self::delete_spec_col(driver, col_name, col_value)
    }

    fn retrieve(
        driver: &mut pg_driver::PgDriver,
        condition: Option<String>,
    ) -> Vec<UserClientCombination> {
        let mut res: Vec<UserClientCombination> = vec![];

        let rows: Vec<Row> = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let user_uuid: String = row.get(UserRepository::get_fk_uuid_name().as_str());
            let client_uuid: String = row.get(ClientRepository::get_fk_uuid_name().as_str());
            res.push(UserClientCombination::new(
                user_uuid.parse().unwrap(),
                client_uuid.parse().unwrap(),
            ))
        }

        res
    }
}
