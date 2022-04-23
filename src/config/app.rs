use crate::controller;
use axum::{routing::IntoMakeService, Extension, Router, Server};
use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use super::Config;

// TODO handle error
pub async fn build_server() -> hyper::Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    let (config, router) = config_router();
    let address: SocketAddr = SocketAddr::new(config.address, config.port);
    let service = router.into_make_service();
    let server = Server::try_bind(&address)?.serve(service);
    hyper::Result::Ok(server)
}

pub fn config_router() -> (Config, Router) {
    config_tracer();
    let config = Config::load().expect("bad config!"); // TODO log properly
    tracing::debug!("{:?}", config);
    let router = Router::new()
        .merge(controller::router())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(config.clone())); // TODO maybe Arc<Config>

    (config, router)
}

fn config_tracer() {
    // TODO study distributed tracing and open telemetry
    // https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/
    // https://docs.rs/tower-http/0.2.5/tower_http/trace/index.html
    tracing_subscriber::registry()
        .with(fmt::layer())
        // TODO EnvFilter::new() so we load from Config struct
        .with(EnvFilter::from_default_env())
        .init();
}
