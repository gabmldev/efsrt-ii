use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Error, Surreal,
};

use crate::config::db::DbEnv;

pub async fn create_client() -> Result<Surreal<Client>, Error> {
    let env = DbEnv::from_env();

    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root",
        password: "secret",
    })
    .await?;

    db.use_ns("ns").use_db("db").await?;
    Ok(db)
}
