use super::Config;
use crate::controller;
use axum::{http::Request, routing::IntoMakeService, Extension, Router, Server};
use figment::Profile;
use hyper::server::conn::AddrIncoming;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use tracing_subscriber::{fmt, prelude::*};
use uuid::Uuid;

pub fn build_server() -> Server<AddrIncoming, IntoMakeService<Router>> {
    let cfg = configure();
    let address = SocketAddr::new(cfg.address, cfg.port);
    let router = build_router(cfg);
    let service = router.into_make_service();
    let server = Server::try_bind(&address)
        .unwrap_or_else(|error| panic!("Error binding to '{}' - {}", address, error))
        .serve(service);

    info!(cfg.address = %address.ip(), cfg.port = %address.port(), "Listening");
    server
}

pub fn configure() -> Config {
    let cfg = Config::load().expect("Unable to parse configuration");
    init_tracer_subscriber(&cfg);
    cfg
}

pub fn configure_for(profile: Profile) -> Config {
    let cfg = Config::load_for(profile).expect("Unable to parse configuration");
    init_tracer_subscriber(&cfg);
    cfg
}

fn init_tracer_subscriber(cfg: &Config) {
    // https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(cfg.new_env_filter())
        .init(); // FIXME change to panic-free try_init()

    info!(%cfg.profile, %cfg.hostname, "Configured");
}

pub fn build_router(cfg: Config) -> Router {
    let hostname = cfg.hostname.clone();
    let trace_layer = TraceLayer::new_for_http().make_span_with(move |_: &Request<_>| {
        let request_id = Uuid::new_v4();
        info_span!("Request", %hostname, %request_id)
    });

    Router::new()
        .merge(controller::router())
        .layer(trace_layer)
        .layer(Extension(cfg))
}
