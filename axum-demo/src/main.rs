#![forbid(unsafe_code)]

use axum_demo::model::Result;

#[tokio::main]
async fn main() -> Result<()> {
    axum_demo::start_server().await
}

// TODO compute test coverage
// TODO graceful shutdown & health probes
// TODO make backend consume two more services: restful and grpc
// TODO extract all reusable parts of the "framework" into a lib
