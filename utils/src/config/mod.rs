// TODO rename to crate::config::server
pub mod app;

pub mod db;
pub mod redis;

// TODO this should become crate::config to avoid inception
#[allow(clippy::module_inception)]
mod config;

pub use config::Config;

pub use figment::Profile;
