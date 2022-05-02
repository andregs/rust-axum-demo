use crate::{
    config::{db, redis, Config},
    controller,
    model::Result,
};
use ::redis::Client;
use axum::{http::Request, Extension, Router};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use uuid::Uuid;

pub async fn new_router(cfg: Config) -> Result<Router> {
    let hostname = cfg.hostname.clone();
    let trace_layer = TraceLayer::new_for_http().make_span_with(move |_: &Request<_>| {
        let request_id = Uuid::new_v4();
        info_span!("Request", %hostname, %request_id)
    });

    let context = Arc::new(ContextData {
        db: db::connect(&cfg).await?,
        redis: redis::open(&cfg)?,
        config: cfg,
    });

    let router = Router::new()
        .merge(controller::router())
        .layer(trace_layer)
        .layer(Extension(context));

    Ok(router)
}

pub struct ContextData {
    pub config: Config,
    pub db: Pool<Postgres>,
    pub redis: Client,
}

pub type AppContext = Extension<Arc<ContextData>>;
