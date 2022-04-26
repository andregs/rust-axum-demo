use super::Config;
use crate::controller;
use axum::{http::Request, routing::IntoMakeService, Extension, Router, Server};
use hyper::server::conn::AddrIncoming;
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use tracing_subscriber::{fmt, prelude::*};
use uuid::Uuid;

pub fn build_server() -> Server<AddrIncoming, IntoMakeService<Router>> {
    let (config, router) = configure();
    let address: SocketAddr = SocketAddr::new(config.address, config.port);
    let service = router.into_make_service();
    Server::try_bind(&address)
        .unwrap_or_else(|e| panic!("Error binding to '{}' - {}", address, e))
        .serve(service)
}

pub fn configure() -> (Config, Router) {
    let config = Config::load().expect("Unable to parse configuration");

    // I could embed this into config struct
    // TODO I'd expect a failure here to panic on prod but not on local
    let pod = env::var("MY_POD_NAME").unwrap_or_else(|_| "unknown".into());

    // https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(config.new_env_filter())
        .init();

    // this log will contain a field with k8s pod's name in addition to the message
    info!(%pod, "Configured with {:?}", config);

    let trace_layer = TraceLayer::new_for_http().make_span_with(move |_: &Request<_>| {
        let request_id = Uuid::new_v4();
        info_span!("request", %pod, %request_id)
    });

    let router = Router::new()
        .merge(controller::router())
        .layer(trace_layer)
        .layer(Extension(config.clone()));

    (config, router)
}
