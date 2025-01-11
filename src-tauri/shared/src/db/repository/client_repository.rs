use chrono::{DateTime, Utc};

use crate::{
    db::{
        db_actions::{DbActions, Table},
        model::client::Client,
    },
    pki_auth_key::PKIAuthenticationKey,
};

pub struct ClientRepository;

impl Table<Client> for ClientRepository {
    fn get_name() -> String {
        String::from("clients")
    }

    fn get_fk_uuid_name() -> String {
        String::from("client_uuid")
    }

    fn get_fmt_cols() -> String {
        String::from("uuid, user_uuid, public_key, device_name, last_used, registered_at")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("user_uuid, public_key, device_name, last_used, registered_at")
    }

    fn get_fmt_vals(model: &Client) -> String {
        format!(
            "'{}', '{}', '{}', '{}', '{}', '{}'",
            model.get_uuid(),
            model.get_user_uuid(),
            model.get_pub_key().to_base64(),
            model.get_device_name(),
            model.get_last_used(),
            model.get_registered_at()
        )
    }

    fn get_fmt_vals_no_id(model: &Client) -> String {
        format!(
            "'{}', '{}', '{}', '{}', '{}'",
            model.get_user_uuid(),
            model.get_pub_key().to_base64(),
            model.get_device_name(),
            model.get_last_used(),
            model.get_registered_at()
        )
    }
}

impl DbActions<Client, Self> for ClientRepository {
    fn store(driver: &mut pg_driver::PgDriver, model: &Client) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(driver: &mut pg_driver::PgDriver, model: &Client) -> anyhow::Result<()> {
        Self::alter(driver, model, model.get_uuid())
    }

    fn remove(driver: &mut pg_driver::PgDriver, model: &Client) -> anyhow::Result<()> {
        Self::delete(driver, model.get_uuid())
    }

    fn retrieve(driver: &mut pg_driver::PgDriver, condition: Option<String>) -> Vec<Client> {
        let mut res = vec![];
        let rows = Self::read(driver, &Self::get_name(), condition);

        for row in rows {
            let uuid = row.get("uuid");
            let user_uuid = row.get("user_uuid");
            let pub_key = PKIAuthenticationKey::from_base64(row.get("public_key")).expect("");
            let name = row.get("device_name");

            let last_used = DateTime::parse_from_rfc3339(row.get("last_used"))
                .map(|dt| dt.with_timezone(&Utc))
                .expect("Invalid timestamp");
            let registered_at = DateTime::parse_from_rfc3339(row.get("registered_at"))
                .map(|dt| dt.with_timezone(&Utc))
                .expect("Invalid timestamp");

            res.push(Client::from(
                uuid,
                user_uuid,
                pub_key,
                name,
                last_used,
                registered_at,
            ))
        }

        res
    }
}
