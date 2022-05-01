use std::time::Duration;

use crate::{config::Config, model::Result};
use anyhow::Context;
use log::LevelFilter;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};

pub async fn connect(cfg: &Config) -> Result<Pool<Postgres>> {
    let mut options: PgConnectOptions = cfg.database_url.parse()?;
    options // ugly config but there's hope https://github.com/launchbadge/sqlx/issues/942
        .log_statements(LevelFilter::Debug)
        .log_slow_statements(LevelFilter::Debug, Duration::from_secs(1)); // FIXME externalize config

    let pool = PgPoolOptions::new()
        .min_connections(0)
        .max_connections(80)
        .connect_timeout(Duration::from_secs(15))
        .idle_timeout(Duration::from_secs(5 * 60))
        .connect_with(options)
        .await
        .context("Unable to connect")?;

    Ok(pool)
}
