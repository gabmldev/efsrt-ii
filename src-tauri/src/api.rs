use std::time::Duration;

use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_SECURITY_POLICY, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct Health {
    status: &'static str,
}
async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

#[derive(Deserialize)]
struct EchoReq {
    messagge: String,
}
#[derive(Serialize)]
struct EchoRes {
    echo: String,
}
async fn echo(Json(p): Json<EchoReq>) -> Json<EchoRes> {
    Json(EchoRes { echo: p.messagge })
}

fn build_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4321".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, CONTENT_SECURITY_POLICY])
        .max_age(Duration::from_secs(12 * 60 * 64));

    Router::new()
        .route("/health", get(health))
        .route("/echo", post(echo))
        .layer(cors)
}

pub async fn run_server(port: u16) {
    let app = build_router();
    let port = format!("{}:{}", "127.0.0.1", port.clone().to_string());
    let listener = TcpListener::bind(port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
