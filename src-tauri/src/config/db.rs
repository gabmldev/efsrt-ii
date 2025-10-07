use crate::{config::utils::get_var, errors::ConfigError};

pub struct DbEnv {
    pub url: String,
    pub user: String,
    pub pwd: String,
    pub ns: String,
    pub db: String,
}

impl DbEnv {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(DbEnv {
            url: get_var("SURREALDB_URL")?,
            user: get_var("SURREALDB_USER")?,
            pwd: get_var("SURREALDB_PWD")?,
            ns: get_var("SURREALDB_NS")?,
            db: get_var("SURREALDB_DB")?,
        })
    }
}
