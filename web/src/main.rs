#![forbid(unsafe_code)]

use axum_demo::config;

#[tokio::main]
async fn main() {
    config::app::build_server().await.unwrap();
}
