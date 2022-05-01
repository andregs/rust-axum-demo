use std::time::Duration;
use tracing::info;
use utils::{
    config::{db, Config},
    model::Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    migrate(Config::load()?).await
}

#[tracing::instrument(
    skip(cfg),
    fields(%cfg.hostname),
)]
async fn migrate(cfg: Config) -> Result<()> {
    let db = db::connect(&cfg).await?;
    info!("Migrating DB...");

    // TODO web app can only be ready after migrations are done
    // I'm faking some long-running migrations here to highlight the issue.
    // Remove this after implementing a mechanism for app deployment to wait for db migrations.
    tokio::time::sleep(Duration::from_secs(7)).await;

    sqlx::migrate!().run(&db).await.map_err(sqlx::Error::from)?;
    info!("DB Migrated.");
    Ok(())
}
