use super::Config;
use sqlx::postgres::{PgConnectOptions, PgConnection};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use sqlx::{ConnectOptions, Connection};

pub async fn connect_pool(cfg: &Config) -> Pool<Postgres> {
    let options = get_pg_options(cfg);
    PgPoolOptions::new()
        .max_connections(5) // FIXME
        .connect_with(options)
        .await
        .expect("Unable to connect") // FIXME
}

pub async fn connect(cfg: &Config) -> PgConnection {
    let options = get_pg_options(cfg);
    PgConnection::connect_with(&options).await.unwrap() // FIXME
}

fn get_pg_options(cfg: &Config) -> PgConnectOptions {
    let mut options: PgConnectOptions = cfg.database_url.parse().unwrap(); // FIXME
    options.disable_statement_logging(); // TODO log at debug or trace level
    options
}
