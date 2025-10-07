use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Error, Surreal,
};

use crate::config::db::DbEnv;

pub async fn create_client() -> Result<Surreal<Client>, Error> {
    let db_cfg = DbEnv::from_env();

    let db_cfg = match db_cfg {
        Ok(db) => db,
        Err(e) => panic!(
            "Failed to load Database configuration from environment variables: {}",
            e
        ),
    };

    let db = Surreal::new::<Ws>(db_cfg.url).await?;

    db.signin(Root {
        username: &db_cfg.user,
        password: &db_cfg.pwd,
    })
    .await?;

    db.use_ns(&db_cfg.ns).use_db(&db_cfg.db).await?;
    Ok(db)
}
