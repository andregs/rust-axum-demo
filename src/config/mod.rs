use super::controller;
use axum::{routing::IntoMakeService, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub async fn build_service() -> (SocketAddr, IntoMakeService<Router>) {
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let service = config_router().into_make_service();
    (address, service)
}

pub fn config_router() -> Router {
    config_tracer();
    Router::new().merge(controller::router()).layer(TraceLayer::new_for_http())
}

fn config_tracer() {
    // TODO study distributed tracing and open telemetry
    // https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/
    // https://docs.rs/tower-http/0.2.5/tower_http/trace/index.html
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}
