#![forbid(unsafe_code)]

use utils::model::Result;

#[tokio::main]
async fn main() -> Result<()> {
    utils::start_server().await
}
