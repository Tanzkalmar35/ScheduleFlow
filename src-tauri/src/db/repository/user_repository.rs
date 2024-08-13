use crate::db::pg_driver::PgDriver;
use crate::db::{
    db_actions::{DbActions, Table},
    model::user::User,
};
use crate::errors::error_messages::USER_NOT_FOUND_ERR;

pub struct UserRepository;

impl UserRepository {
    /// Checks if a user with a given email already exists.
    ///
    /// # Returns
    /// True, if there is a user with the given email, otherwise false.
    ///
    /// TODO: refactor to using EXISTS() for performance
    pub(crate) fn is_existing(driver: &mut PgDriver, email: &str) -> bool {
        let condition = format!("email = '{}'", email);

        let res = Self::retrieve(driver, Some(condition));

        !res.is_empty()
    }

    pub(crate) fn get_by_email(driver: &mut PgDriver, email: String) -> Result<User, &'static str> {
        let condition = format!("email = '{}'", email);
        let user_opt = UserRepository::retrieve(driver, Some(condition))
            .get(0)
            .cloned();
        if let Some(user) = user_opt {
            Ok(user)
        } else {
            Err(USER_NOT_FOUND_ERR)
        }
    }
}

impl Table<User> for UserRepository {
    fn get_name() -> String {
        String::from("users")
    }

    fn get_fk_uuid_name() -> String {
        String::from("user_uuid")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, username, password, email")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("username, password, email")
    }

    fn get_fmt_vals(user: &User) -> String {
        format!(
            "'{}', '{}', '{}', '{}'",
            &user.get_uuid(),
            &user.get_username(),
            &user.get_password(),
            &user.get_email()
        )
    }

    fn get_fmt_vals_no_id(user: &User) -> String {
        format!(
            "'{}', '{}', '{}'",
            &user.get_username(),
            &user.get_password(),
            &user.get_email()
        )
    }
}

impl DbActions<User, Self> for UserRepository {
    fn store(driver: &mut PgDriver, model: User) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(driver: &mut PgDriver, model: User) -> anyhow::Result<()> {
        Self::alter(driver, &model, model.get_uuid())
    }

    fn remove(driver: &mut PgDriver, model: User) -> anyhow::Result<()> {
        Self::delete(driver, model.get_uuid())
    }

    fn retrieve(driver: &mut PgDriver, condition: Option<String>) -> Vec<User> {
        let mut res: Vec<User> = vec![];

        let rows = Self::read(driver, &Self::get_name(), condition);

        for row in rows {
            res.push(User::from(
                row.get("uuid"),
                row.get("username"),
                row.get("email"),
                row.get("password"),
            ));
        }

        res
    }
}
