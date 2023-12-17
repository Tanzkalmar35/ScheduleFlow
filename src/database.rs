use anyhow::Result;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

type Database = Surreal<surrealdb::engine::local::Db>;

struct SurrealDb {
    datastore: Database,
}

impl SurrealDb {}

#[derive(Default)]
struct SurrealDbBuilder {
    database: Option<Database>,
}

impl SurrealDbBuilder {
    pub fn new() -> Self {
        SurrealDbBuilder::default()
    }

    async fn build() -> Result<SurrealDb> {
        todo!()
    }
}
