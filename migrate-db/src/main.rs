use std::time::Duration;

use sqlx::postgres::{PgConnectOptions, PgConnection};
use sqlx::{ConnectOptions, Connection};
use tracing::info_span;
use utils::config;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let app_config = config::app::configure();

    let mut db_config: PgConnectOptions = app_config.database_url.parse()?;
    db_config.disable_statement_logging();

    let db = &mut PgConnection::connect_with(&db_config).await?;
    info_span!("Migration", "Migrating DB...");
    sqlx::migrate!().run(db).await?;

    // TODO web app can only be ready after migrations are done
    tokio::time::sleep(Duration::from_secs(12)).await; // remove me

    info_span!("Migration", "DB Migrated.");
    Ok(())
}
