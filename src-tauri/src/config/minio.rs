use std::{env, path::PathBuf};

use crate::{
    config::utils::{get_bool, get_var},
    errors::ConfigError,
};

pub struct MinioEnv {
    pub url: String,
    pub access_key: String,
    pub secret_key: String,
    pub certificate_path: Option<PathBuf>,
    pub check_certificate: Option<bool>,
    pub session_token: Option<String>,
}

impl MinioEnv {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            url: get_var("MINIO_URL")?,
            access_key: get_var("MINIO_ACCESS_KEY")?,
            secret_key: get_var("MINIO_SECRET_KEY")?,
            session_token: get_var("MINIO_SESSION_TOKEN").ok(),
            certificate_path: env::var("MINIO_CERTIFICATE_PATH").ok().map(PathBuf::from),
            check_certificate: get_bool("MINIO_CHECK_CERTIFICATE").ok(),
        })
    }
}
