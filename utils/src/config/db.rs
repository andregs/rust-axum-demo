use crate::{config::Config, model::Result};
use anyhow::Context;
use log::LevelFilter;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};
use std::time::Duration;

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
