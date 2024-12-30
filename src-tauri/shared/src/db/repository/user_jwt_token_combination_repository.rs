use crate::db::{
    db_actions::{DbActions, Table},
    model::user_jwt_token_combination::UserJwtTokenCombination,
    repository::{jwt_token_repository::JwtTokenRepository, user_repository::UserRepository},
};
use pg_driver::PgDriver;
use postgres::Row;

pub struct UserJwtTokenCombinationRepository;

impl Table<UserJwtTokenCombination> for UserJwtTokenCombinationRepository {
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

    fn get_fmt_vals(model: &UserJwtTokenCombination) -> String {
        format!("'{}', '{}'", model.user_uuid, model.jwt_token_uuid)
    }

    fn get_fmt_vals_no_id(_model: &UserJwtTokenCombination) -> String {
        "".to_string()
    }
}

impl DbActions<UserJwtTokenCombination, Self> for UserJwtTokenCombinationRepository {
    fn store(driver: &mut PgDriver, model: &UserJwtTokenCombination) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(_driver: &mut PgDriver, _model: &UserJwtTokenCombination) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(driver: &mut PgDriver, model: &UserJwtTokenCombination) -> anyhow::Result<()> {
        let col_name: String = UserRepository::get_fk_uuid_name();
        let col_value: String = model.user_uuid.to_string();
        Self::delete_spec_col(driver, col_name, col_value)
    }

    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<UserJwtTokenCombination> {
        let mut res: Vec<UserJwtTokenCombination> = vec![];

        let rows: Vec<Row> = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let calendar_uuid: String = row.get(UserRepository::get_fk_uuid_name().as_str());
            let property_uuid: String = row.get(JwtTokenRepository::get_fk_uuid_name().as_str());
            res.push(UserJwtTokenCombination::new(
                calendar_uuid.parse().unwrap(),
                property_uuid.parse().unwrap(),
            ))
        }

        res
    }
}
