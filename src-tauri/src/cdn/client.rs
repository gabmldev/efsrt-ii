use crate::{config::minio::MinioEnv, errors::ConfigError};
use minio::s3::{creds::StaticProvider, http::BaseUrl, Client};

pub async fn create_client() -> Client {
    let minio_cfg = MinioEnv::from_env();

    let minio_cfg = match minio_cfg {
        Ok(v) => {
            println!("MinIO configuration loaded from environment variables.");
            v
        }
        Err(e) => {
            panic!(
                "Failed to load MinIO configuration from environment variables: {}",
                e
            );
        }
    };

    let creds = StaticProvider::new(
        &minio_cfg.access_key.clone(),
        &minio_cfg.secret_key.clone(),
        minio_cfg.session_token.as_deref().clone(),
    );

    let base_url: BaseUrl = minio_cfg.url.parse().unwrap();
    let cert_path: Option<std::path::PathBuf> = minio_cfg.certificate_path.clone();
    let client: Client = Client::new(
        base_url,
        Some(Box::new(creds)),
        cert_path.as_ref().map(|cp| cp.as_path()),
        None,
    )
    .map_err(ConfigError::MinIO)
    .unwrap();

    client
}
