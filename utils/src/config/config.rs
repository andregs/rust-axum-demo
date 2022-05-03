use crate::model::Result;
use anyhow::Context;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    value::{Dict, Map},
    Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    net::{IpAddr, Ipv4Addr},
};
use tracing::{info, subscriber, warn};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub profile: Profile,
    pub hostname: String,
    pub address: IpAddr,
    pub port: u16,
    pub log_level: String,
    pub db_username: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub redis_url: String,
    pub slow_sql_seconds: u64,
    pub db_pool_min_connections: u32,
    pub db_pool_max_connections: u32,
    pub db_pool_connect_timeout: u64,
    pub db_pool_idle_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            profile: Profile::Default,
            hostname: hostname(),
            address: Ipv4Addr::LOCALHOST.into(),
            port: 3000,
            log_level: "info,tower_http=debug".into(),
            db_username: "stanley".into(),
            db_password: "ipkiss".into(),
            db_host: "localhost".into(),
            db_port: 5432,
            db_name: "axum_demo".into(),
            redis_url: "redis://localhost:6379".into(),
            slow_sql_seconds: 1,
            db_pool_min_connections: 0,
            db_pool_max_connections: 80,
            db_pool_connect_timeout: 15,
            db_pool_idle_timeout: 5 * 60,
        }
    }
}

fn hostname() -> String {
    hostname::get()
        .unwrap_or_else(|_| OsString::from("where-am-i"))
        .to_string_lossy()
        .into_owned() // why rust, why!
}

impl Config {
    pub fn load() -> Result<Self> {
        let profile = Profile::from_env_or("APP_PROFILE", Profile::Default);
        Self::load_for(profile)
    }

    pub fn load_for(profile: Profile) -> Result<Self> {
        let cfg = Figment::from(Config::default())
            .merge(Toml::file("application.toml").nested())
            .merge(Env::prefixed("APP_").global())
            .select(profile)
            .extract::<Config>()
            .context("Unable to parse app configuration")?;

        if let Err(err) = LogTracer::init() {
            warn!(%cfg.profile, %cfg.hostname, %err, "Log tracer init failure");
        }

        // https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/
        let filter = EnvFilter::try_new(&cfg.log_level).context("Unable to set the log level")?;
        let subscriber = tracing_subscriber::registry().with(fmt::layer()).with(filter);
        if let Err(err) = subscriber::set_global_default(subscriber) {
            warn!(%cfg.profile, %cfg.hostname, %err, "Tracing subscriber init failure");
        }

        info!(%cfg.profile, %cfg.hostname, "Configured");
        Ok(cfg)
    }
}

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("App Config")
    }

    fn data(&self) -> figment::error::Result<Map<Profile, Dict>> {
        Serialized::defaults(self).data()
    }
}
