use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

use anyhow::Result;

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
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("test").use_db("test").await?;
        
        Ok(
            SurrealDb {
                datastore: db,
            }
          )


        //let created = db
        //    .create("person")
        //    .content(
        //        Person::new("Mr.", "John Doe", true)
        //    ).await?;
        //dbg!(created);
    }
}
