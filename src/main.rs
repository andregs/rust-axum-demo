#![forbid(unsafe_code)]

mod config;
mod controller;
mod model;

#[tokio::main]
async fn main() {
    let (address, service) = config::build_app().await;
    tracing::info!("listening on {}", address);
    axum::Server::bind(&address).serve(service).await.unwrap();
}
