use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Variable missing or invalid environment: {0}")]
    EnvVar(String),

    #[error("Dotenv error, maybe env not set or not exists")]
    Dotenv(#[from] dotenvy::Error),

    #[error("Database error: {0:?}")]
    SurrealDb(#[from] surrealdb::Error),

    #[error("MinIO error: {0:?}")]
    MinIO(#[from] minio::s3::error::Error),
}
