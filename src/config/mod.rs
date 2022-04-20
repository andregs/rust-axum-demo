use super::controller;
use axum::{routing::IntoMakeService, Router};
use std::net::SocketAddr;

pub async fn build_app() -> (SocketAddr, IntoMakeService<Router>) {
    config_tracer();
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let service = config_router().into_make_service();
    (address, service)
}

fn config_tracer() {
    tracing_subscriber::fmt::init();
}

fn config_router() -> Router {
    Router::new().merge(controller::router())
}
