#![forbid(unsafe_code)]

use crate::{
    config::{context, Config},
    model::*,
};
use anyhow::Context;
use axum::Server;
use std::net::SocketAddr;
use tracing::info;

pub mod config;
mod controller;
pub mod model;
mod validation;
mod credential_repo;
mod service;
mod token_repo;

pub async fn start_server() -> Result<()> {
    let cfg = Config::load()?;
    let address = SocketAddr::new(cfg.address, cfg.port);
    let router = context::new_router(cfg).await?;

    info!(cfg.address = %address.ip(), cfg.port = %address.port(), "Starting server");
    Server::try_bind(&address)
        .with_context(|| format!("Unable to bind to {}", &address))?
        .serve(router.into_make_service())
        .await
        .context("HTTP server error")?;

    info!(cfg.address = %address.ip(), cfg.port = %address.port(), "Bye!"); // this is never called
    Ok(())
}
