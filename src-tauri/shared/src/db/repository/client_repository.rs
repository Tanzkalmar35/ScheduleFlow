use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{DateTime, NaiveDateTime, Utc};
use customs::bench_message;
use ed25519_dalek::VerifyingKey;

use crate::db::{
    db_actions::{DbActions, Table},
    model::client::Client,
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
            STANDARD.encode(model.get_pub_key().to_bytes()),
            model.get_device_name(),
            model.get_last_used(),
            model.get_registered_at()
        )
    }

    fn get_fmt_vals_no_id(model: &Client) -> String {
        format!(
            "'{}', '{}', '{}', '{}', '{}'",
            model.get_user_uuid(),
            STANDARD.encode(model.get_pub_key().to_bytes()),
            model.get_device_name(),
            model.get_last_used(),
            model.get_registered_at()
        )
    }
}

impl DbActions<Client, Self> for ClientRepository {
    #[bench_message("Storing calendar")]
    fn store(driver: &mut pg_driver::PgDriver, model: &Client) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    #[bench_message("Updating calendar")]
    fn update(driver: &mut pg_driver::PgDriver, model: &Client) -> anyhow::Result<()> {
        Self::alter(driver, model, model.get_uuid())
    }

    #[bench_message("Deleting calendar")]
    fn remove(driver: &mut pg_driver::PgDriver, model: &Client) -> anyhow::Result<()> {
        Self::delete(driver, model.get_uuid())
    }

    #[bench_message("Retrieving clients")]
    fn retrieve(driver: &mut pg_driver::PgDriver, condition: Option<String>) -> Vec<Client> {
        let mut res = vec![];
        let rows = Self::read(driver, &Self::get_name(), condition);

        for row in rows {
            let uuid = row.get("uuid");
            let user_uuid = row.get("user_uuid");
            let pub_key_str: String = row.get("public_key");
            let pub_key =
                VerifyingKey::try_from(&STANDARD.decode(pub_key_str).unwrap()[..32]).unwrap(); // TODO: Improve error handling
            let name = row.get("device_name");
            let last_used: NaiveDateTime = row.get("last_used");
            let registered_at: NaiveDateTime = row.get("registered_at");

            res.push(Client::from(
                uuid,
                user_uuid,
                pub_key,
                name,
                DateTime::from_naive_utc_and_offset(last_used, Utc),
                DateTime::from_naive_utc_and_offset(registered_at, Utc),
            ))
        }

        res
    }
}
