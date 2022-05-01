#![forbid(unsafe_code)]

use utils::config;

#[tokio::main]
async fn main() {
    config::app::build_server()
        .await // build is async because it has to e.g. connect to DB
        .await // here we start the HTTP server
        .expect("I have no idea why this has failed.");
}
