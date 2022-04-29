#![forbid(unsafe_code)]

use utils::config;

#[tokio::main]
async fn main() {
    config::app::build_server().await.unwrap();
}
