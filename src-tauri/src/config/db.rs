use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Variable missing or invalid environment: {0}")]
    EnvVar(String),

    #[error("Dotenv error, maybe env not set or not exists")]
    Dotenv(#[from] dotenvy::Error),

    #[error("Database error: {0:?}")]
    Db(#[from] surrealdb::Error),
}

pub struct DbEnv {
    surrealdb_url: String,
    surrealdb_user: String,
    surrealdb_pwd: String,
    surrealdb_ns: String,
    surrealdb_db: String,
}

impl DbEnv {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        Ok(DbEnv {
            surrealdb_url: get_var("SURREALDB_URL")?,
            surrealdb_user: get_var("SURREALDB_USER")?,
            surrealdb_pwd: get_var("SURREALDB_PWD")?,
            surrealdb_ns: get_var("SURREALDB_NS")?,
            surrealdb_db: get_var("SURREALDB_DB")?,
        })
    }
}

fn get_var(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::EnvVar(key.into()))
}
