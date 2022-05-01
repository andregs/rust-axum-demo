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
use tracing_subscriber::EnvFilter;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub profile: Profile,
    pub hostname: String,
    pub address: IpAddr,
    pub port: u16,
    pub log_level: String,
    pub database_url: String,
    pub redis_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            profile: Profile::Default,
            hostname: hostname(),
            address: Ipv4Addr::LOCALHOST.into(),
            port: 3000,
            log_level: "info,tower_http=debug".into(),
            database_url: "postgres://localhost/axum_demo".into(),
            redis_url: "redis://localhost:6379".into(),
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
    pub fn load() -> Result<Self, figment::Error> {
        let profile = Profile::from_env_or("APP_PROFILE", Profile::Default);
        Self::load_for(profile)
    }

    pub fn load_for(profile: Profile) -> Result<Self, figment::Error> {
        Figment::from(Config::default())
            .merge(Toml::file("application.toml").nested())
            .merge(Env::prefixed("APP_").global())
            .select(profile)
            .extract::<Config>()
    }

    pub fn new_env_filter(&self) -> EnvFilter {
        EnvFilter::try_new(&self.log_level).expect("Unable to set the log level")
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
