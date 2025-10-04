use minio::s3::{creds::StaticProvider, http::BaseUrl, Client};

pub fn create_client() -> Client {
    // BASE_URL
    let base_url: BaseUrl = "play.min.io".parse().unwrap();

    let static_provider = StaticProvider::new(
        // ACCESS_KEY
        "Q3AM3UQ867SPQQA43P2F",
        // SECRET_KEY
        "zuf+tfteSlswRu7BJ86wekitnifILbZam1KYY3TG",
        // SESSION_TOKEN <optional>
        None,
    );

    let certificate_path = "./certf";
    let check_certificate: bool = true;

    let client: Client = Client::new(
        base_url,
        Some(Box::new(static_provider)),
        None, // certificate_path,
        None, //check_certificate,
    )
    .unwrap();

    client
}
