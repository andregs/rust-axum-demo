#![forbid(unsafe_code)]

use axum_demo::config;

#[tokio::main]
async fn main() {
    config::app::build_server()
        .await
        .expect("server error")
        .await
        .expect("one more");
}
