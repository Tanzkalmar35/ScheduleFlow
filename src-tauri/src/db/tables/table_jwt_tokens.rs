use std::fmt::format;
use serde::Serialize;

use crate::db_actions::{DbActions, Table};
use crate::pg_driver::PgDriver;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct JwtToken {
    pub(crate) token: String,
    pub(crate) user_uuid: Uuid,
}

impl JwtToken {
    pub fn new(token: String, user_uuid: Uuid) -> Self {
        Self { token, user_uuid }
    }
}

impl Table for &JwtToken {
    fn get_name() -> String {
        String::from("user_jwt_tokens")
    }

    fn get_fmt_cols() -> String {
        String::from("token, user_uuid")
    }

    fn get_fk_uuid_name() -> String {
        String::from("jwt_token")
    }

    fn get_fmt_cols_no_id() -> String {
        Self::get_fmt_cols()
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}', '{}'", self.token, self.user_uuid)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        self.get_fmt_vals()
    }
}

impl Table for JwtToken {
    fn get_name() -> String {
        String::from("user_jwt_tokens")
    }

    fn get_fmt_cols() -> String {
        String::from("token, user_uuid")
    }

    fn get_fk_uuid_name() -> String {
        String::from("jwt_token")
    }

    fn get_fmt_cols_no_id() -> String {
        Self::get_fmt_cols()
    }

    fn get_fmt_vals(&self) -> String {
        format!("'{}', '{}'", self.token, self.user_uuid)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        self.get_fmt_vals()
    }
}

impl DbActions for JwtToken {
    type Item = Self;

    fn store(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::insert(driver, self)
    }

    fn update(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        unimplemented!("JWT's are not supposed to be updated")
    }

    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::delete_spec_col::<&Self>(driver, String::from("token"), self.token.clone())
    }

    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<Self::Item> {
        let mut res: Vec<Self> = vec![];

        if let Ok(rows) = Self::read(driver, Self::get_name().as_str(), condition) {
            for row in rows {
                let token = row.get("token");
                let user_uuid = row.get("user_uuid");
                res.push(Self::new(token, user_uuid))
            };
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pg_driver::PgDriver;
    use uuid::Uuid;
    use crate::table_users::User;

    #[test]
    fn test_store() {
        let mut driver = PgDriver::setup();
        assert!(driver.connect().is_ok());

        let user = User::new("username".to_string(), "email".to_string(), "pass".to_string());
        assert!(user.store(&mut driver).is_ok());

        let token = JwtToken::new(String::from("test_token1"), user.uuid);
        let result = token.store(&mut driver);

        assert!(result.is_ok())
    }

    #[test]
    fn test_remove() {
        let mut driver = PgDriver::setup();
        assert!(driver.connect().is_ok());

        let user = User::new("username".to_string(), "email".to_string(), "pass".to_string());
        assert!(user.store(&mut driver).is_ok());

        let token = JwtToken::new(String::from("test_token2"), user.uuid);
        token.store(&mut driver).unwrap();

        let result = token.remove(&mut driver);

        assert!(result.is_ok());
    }

    #[test]
    fn test_retrieve() {
        let mut driver = PgDriver::setup();
        assert!(driver.connect().is_ok());

        let user = User::new("username".to_string(), "email".to_string(), "pass".to_string());
        assert!(user.store(&mut driver).is_ok());

        let token = JwtToken::new(String::from("test_token3"), user.uuid);
        token.store(&mut driver).unwrap();

        let retrieved_tokens = JwtToken::retrieve(&mut driver, None);

        assert!(!retrieved_tokens.is_empty());
    }
}
