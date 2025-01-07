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
        String::from("uuid, name")
    }

    fn get_fmt_cols_no_id() -> String {
        String::from("name")
    }

    fn get_fmt_vals(client: &Client) -> String {
        format!("'{}', '{}'", client.uuid, client.name)
    }

    fn get_fmt_vals_no_id(client: &Client) -> String {
        format!("'{}'", client.name)
    }
}

impl DbActions<Client, Self> for ClientRepository {
    fn store(driver: &mut pg_driver::PgDriver, model: &Client) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(driver: &mut pg_driver::PgDriver, model: &Client) -> anyhow::Result<()> {
        Self::alter(driver, model, model.uuid)
    }

    fn remove(driver: &mut pg_driver::PgDriver, model: &Client) -> anyhow::Result<()> {
        Self::delete(driver, model.uuid)
    }

    fn retrieve(driver: &mut pg_driver::PgDriver, condition: Option<String>) -> Vec<Client> {
        let mut matches: Vec<Client> = vec![];

        let rows = Self::read(driver, &Self::get_name(), condition);

        for row in rows {
            matches.push(Client::from(row.get("uuid"), row.get("name")));
        }

        matches
    }
}
