use super::Config;
use crate::model::Result;
use anyhow::Context;
use redis::Client;

pub fn open(config: &Config) -> Result<Client> {
    let redis_url = config.redis_url.as_str();
    let client = Client::open(redis_url).context("Redis URL check has failed")?;
    Ok(client)
}
