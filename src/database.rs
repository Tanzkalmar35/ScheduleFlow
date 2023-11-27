use surrealdb::dbs::Session;
use surrealdb::engine::local::Mem;
use surrealdb::kvs::Datastore;
use surrealdb::Surreal;

struct SurrealDb {
    datastore: Datastore,
    session: Session,
}

impl SurrealDb {}

#[derive(Default)]
struct SurrealDbBuilder {
    datastore: Option<Datastore>,
    session: Option<Session>,
}

#[derive(Debug)]
struct Person<'a> {
    title: &'a str,
    name: String,
    marketing: bool,
}

impl SurrealDbBuilder {
    pub fn new() -> Self {
        SurrealDbBuilder::default()
    }

    async fn build() -> SurrealDb {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("test").use_db("test").await?;

        SurrealDb {
            datastore: db.datastore,
            session: db.session,
        }

        //let created = db
        //    .create("person")
        //    .content(
        //        Person::new("Mr.", "John Doe", true)
        //    ).await?;
        //dbg!(created);
    }
}
