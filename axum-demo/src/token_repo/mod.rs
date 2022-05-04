use crate::model::{Error, Result, Token};
use anyhow::Context;
use axum::async_trait;
use redis::{AsyncCommands, Client};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TokenRepoApi {
    async fn save_token(&self, token: &Token, username: &str) -> Result<()>;
    async fn get_username(&self, token: &Token) -> Result<String>;
}

pub struct RedisTokenRepo {
    client: Client,
}

impl RedisTokenRepo {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }
}

#[async_trait]
impl TokenRepoApi for RedisTokenRepo {
    async fn save_token(&self, token: &Token, username: &str) -> Result<()> {
        // redis-rs currently doesn't have connection pooling
        let mut conn = self
            .client
            .get_async_connection()
            .await
            .context("Unable to connect to Redis")?;
        let key = get_key(token);
        let value = username;
        conn.set(key, value).await.context("Unable to store the token")?;
        Ok(())
    }

    async fn get_username(&self, token: &Token) -> Result<String> {
        let mut conn = self
            .client
            .get_async_connection()
            .await
            .context("Unable to connect to Redis")?;
        let key = get_key(token);
        let value: Option<String> = conn.get(key).await.context("Unable to fetch the username")?;
        value.ok_or(Error::BadToken)
    }
}

fn get_key(token: &Token) -> String {
    format!("token:{}", token)
}

#[cfg(test)]
mod tests;
