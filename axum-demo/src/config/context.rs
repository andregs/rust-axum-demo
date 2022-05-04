use super::Config;
use crate::{controller, model::Result};
use ::redis::Client;
use anyhow::Context;
use axum::{http::Request, Extension, Router};
use log::LevelFilter;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};
use std::sync::Arc;
use std::time::Duration;
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

pub mod db {
    use super::*;

    pub async fn connect(cfg: &Config) -> Result<Pool<Postgres>> {
        let slow_sql_seconds = Duration::from_secs(cfg.slow_sql_seconds);
        let db_pool_connect_timeout = Duration::from_secs(cfg.db_pool_connect_timeout);
        let db_pool_idle_timeout = Duration::from_secs(cfg.db_pool_idle_timeout);

        let mut options = PgConnectOptions::new()
            .username(cfg.db_username.as_str())
            .password(cfg.db_password.as_str())
            .host(cfg.db_host.as_str())
            .port(cfg.db_port)
            .database(cfg.db_name.as_str());

        options // ugly config but there's hope https://github.com/launchbadge/sqlx/issues/942
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Debug, slow_sql_seconds);

        let pool = PgPoolOptions::new()
            .min_connections(cfg.db_pool_min_connections)
            .max_connections(cfg.db_pool_max_connections)
            .connect_timeout(db_pool_connect_timeout)
            .idle_timeout(db_pool_idle_timeout)
            .connect_with(options)
            .await
            .context("Unable to connect")?;

        Ok(pool)
    }
}

pub mod redis {
    use super::*;

    pub fn open(config: &Config) -> Result<Client> {
        let redis_url = config.redis_url.as_str();
        let client = Client::open(redis_url).context("Redis URL check has failed")?;
        Ok(client)
    }
}
