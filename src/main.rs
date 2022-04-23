#![forbid(unsafe_code)]

use axum_demo::config;

#[tokio::main]
async fn main() {
    let (address, service) = config::build_service().await;
    tracing::info!("listening on {}", address);
    axum::Server::bind(&address).serve(service).await.unwrap();
}
