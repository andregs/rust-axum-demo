use std::time::Duration;
use tracing::info;
use utils::config::{app, db, Config};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    migrate(app::configure()).await
}

#[tracing::instrument(
    skip(cfg),
    fields(%cfg.hostname),
)]
async fn migrate(cfg: Config) -> Result<(), sqlx::Error> {
    let db = &mut db::connect(&cfg).await;
    info!("Migrating DB...");

    // faking some long-running migrations here
    // TODO web app can only be ready after migrations are done
    tokio::time::sleep(Duration::from_secs(7)).await; // remove me

    sqlx::migrate!().run(db).await?;
    info!("DB Migrated.");
    Ok(())
}
