use crate::db::db_actions::{DbActions, Table};
use crate::db::model::jwt_token::JwtToken;
use pg_driver::PgDriver;

pub struct JwtTokenRepository;

impl JwtTokenRepository {}

impl Table<JwtToken> for JwtTokenRepository {
    fn get_name() -> String {
        String::from("user_jwt_tokens")
    }

    fn get_fk_uuid_name() -> String {
        String::from("jwt_token")
    }

    fn get_fmt_cols() -> String {
        String::from("token, user_uuid")
    }

    fn get_fmt_cols_no_id() -> String {
        Self::get_fmt_cols()
    }

    fn get_fmt_vals(token: &JwtToken) -> String {
        format!("'{}', '{}'", token.token, token.user_uuid)
    }

    fn get_fmt_vals_no_id(token: &JwtToken) -> String {
        Self::get_fmt_vals(token)
    }
}

impl DbActions<JwtToken, Self> for JwtTokenRepository {
    fn store(driver: &mut PgDriver, token: &JwtToken) -> anyhow::Result<()> {
        Self::insert(driver, token)
    }

    fn update(_driver: &mut PgDriver, _token: &JwtToken) -> anyhow::Result<()> {
        unimplemented!("JWT's are not supposed to be updated")
    }

    fn remove(driver: &mut PgDriver, token: &JwtToken) -> anyhow::Result<()> {
        Self::delete_spec_col(driver, String::from("token"), token.token.clone())
    }

    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<JwtToken> {
        let mut res: Vec<JwtToken> = vec![];

        let rows = Self::read(driver, &Self::get_name(), condition);

        for row in rows {
            let token = row.get("token");
            let user_uuid = row.get("user_uuid");
            res.push(JwtToken::new(token, user_uuid))
        }

        res
    }
}
