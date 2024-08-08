use crate::db::{
    db_actions::{DbActions, Table},
    model::user::User,
};

pub struct UserRepository;

impl Table<User> for UserRepository {
    fn get_name() -> String {
        String::from("users")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, username, password, email")
    }

    fn get_fk_uuid_name() -> String {
        String::from("user_uuid")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("username, password, email")
    }

    fn get_fmt_vals(model: &User) -> String {
        format!(
            "'{}', '{}', '{}', '{}'",
            &model.uuid, &model.username, &model.password, &model.email
        )
    }

    fn get_fmt_vals_no_id(model: &User) -> String {
        format!(
            "'{}', '{}', '{}'",
            &model.username, &model.password, &model.email
        )
    }
}

impl DbActions<User, Self> for UserRepository {
    fn store(driver: &mut crate::db::pg_driver::PgDriver, model: User) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(driver: &mut crate::db::pg_driver::PgDriver, model: User) -> anyhow::Result<()> {
        Self::alter(driver, &model, model.uuid)
    }

    fn remove(driver: &mut crate::db::pg_driver::PgDriver, model: User) -> anyhow::Result<()> {
        Self::delete(driver, model.uuid)
    }

    fn retrieve(
        driver: &mut crate::db::pg_driver::PgDriver,
        condition: Option<String>,
    ) -> Vec<User> {
        let mut res: Vec<User> = vec![];

        let rows = Self::read(driver, "users", condition);

        for row in rows {
            res.push(User {
                uuid: row.get("uuid"),
                username: row.get("username"),
                password: row.get("password"),
                email: row.get("email"),
            })
        }

        res
    }
}
