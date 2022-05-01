use crate::{config::*, controller};
use axum::{http::Request, routing::IntoMakeService, Extension, Router, Server};
pub use figment::Profile;
use hyper::server::conn::AddrIncoming;
use sqlx::{Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, warn};
use tracing_subscriber::{fmt, prelude::*};
use uuid::Uuid;

pub async fn build_server() -> Server<AddrIncoming, IntoMakeService<Router>> {
    let cfg = configure();
    let address = SocketAddr::new(cfg.address, cfg.port);
    let router = build_router(cfg).await;
    let service = router.into_make_service();
    let server = Server::try_bind(&address)
        .unwrap_or_else(|error| panic!("Error binding to '{}' - {}", address, error))
        .serve(service);

    info!(cfg.address = %address.ip(), cfg.port = %address.port(), "Listening");
    server
}

pub async fn build_router(cfg: Config) -> Router {
    let hostname = cfg.hostname.clone();
    let trace_layer = TraceLayer::new_for_http().make_span_with(move |_: &Request<_>| {
        let request_id = Uuid::new_v4();
        info_span!("Request", %hostname, %request_id)
    });

    let context = Arc::new(Context {
        db: db::connect_pool(&cfg).await,
        config: cfg,
    });

    Router::new()
        .merge(controller::router())
        .layer(trace_layer)
        .layer(Extension(context))
}

pub struct Context {
    pub db: Pool<Postgres>,
    pub config: Config,
}

pub type AppContext = Extension<Arc<Context>>;

pub fn configure() -> Config {
    let cfg = Config::load().expect("Unable to parse configuration");
    init_tracer_subscriber(&cfg);
    cfg
}

// TODO create our own Profile enum
pub fn configure_for(profile: Profile) -> Config {
    let cfg = Config::load_for(profile).expect("Unable to parse configuration");
    init_tracer_subscriber(&cfg);
    cfg
}

fn init_tracer_subscriber(cfg: &Config) {
    // https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/
    let result = tracing_subscriber::registry()
        .with(fmt::layer())
        .with(cfg.new_env_filter())
        .try_init();

    match result {
        Ok(_) => info!(%cfg.profile, %cfg.hostname, "Configured"),
        Err(err) => warn!(%cfg.profile, %cfg.hostname, %err, "Tracing subscriber init failure"),
    }
}
