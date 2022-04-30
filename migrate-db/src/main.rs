use sqlx::postgres::{PgConnectOptions, PgConnection};
use sqlx::{ConnectOptions, Connection};
use std::time::Duration;
use tracing::info;
use utils::config::{app, Config};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    migrate(app::configure()).await
}

#[tracing::instrument(
    skip(cfg),
    fields(%cfg.hostname),
)]
async fn migrate(cfg: Config) -> Result<(), sqlx::Error> {
    let mut db_config: PgConnectOptions = cfg.database_url.parse()?;
    db_config.disable_statement_logging();

    let db = &mut PgConnection::connect_with(&db_config).await?;
    info!("Migrating DB...");

    // faking some long-running migrations here
    // TODO web app can only be ready after migrations are done
    tokio::time::sleep(Duration::from_secs(7)).await; // remove me

    sqlx::migrate!().run(db).await?;
    info!("DB Migrated.");
    Ok(())
}
