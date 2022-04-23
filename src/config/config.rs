use figment::{
    providers::{Env, Format, Serialized, Toml},
    value::{Dict, Map},
    Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use tracing::Level;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub profile: Profile,
    pub address: IpAddr,
    pub port: u16,
    pub log_level: String,
    pub database_url: String,
    pub redis_url: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            profile: Profile::Default,
            address: Ipv4Addr::LOCALHOST.into(),
            port: 3000,
            log_level: Level::DEBUG.to_string(),
            database_url: "postgres://postgres:mysecretpassword@localhost/axum_demo".into(),
            redis_url: "redis://localhost:6379".into(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, figment::Error> {
        let profile = Profile::from_env_or("APP_PROFILE", Profile::Default);
        Figment::from(Config::default())
            .merge(Toml::file("App.toml").nested())
            .merge(Env::prefixed("APP_").global())
            .select(profile)
            .extract::<Config>()
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
