use crate::{credential_repo::*, model::*, token_repo::*};
use axum::async_trait;
use redis::Client;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[async_trait]
pub trait AuthServiceApi {
    async fn register(&self, credentials: Credentials) -> Result<i64>;
    async fn login(&self, credentials: Credentials) -> Result<Token>;
    async fn authenticate(&self, token: Token) -> Result<String>;
}

pub struct AuthService<CR = PostgresCredentialRepo, TR = RedisTokenRepo>
where
    CR: CredentialRepoApi,
    TR: TokenRepoApi,
{
    db: Pool<Postgres>,
    credential_repo: CR,
    token_repo: TR,
}

impl AuthService {
    pub fn new(db_pool: &Pool<Postgres>, redis_client: &Client) -> Self {
        Self {
            db: db_pool.clone(),
            credential_repo: PostgresCredentialRepo,
            token_repo: RedisTokenRepo::new(redis_client),
        }
    }
}

#[async_trait]
impl<CR, TR> AuthServiceApi for AuthService<CR, TR>
where
    CR: CredentialRepoApi + Sync + Send,
    TR: TokenRepoApi + Sync + Send,
{
    async fn register(&self, credentials: Credentials) -> Result<i64> {
        let mut tx = self.db.begin().await?;
        let new_id = self.credential_repo.insert_credentials_tx(&mut tx, &credentials).await;

        match new_id {
            Ok(_) => tx.commit().await?,
            Err(_) => tx.rollback().await?,
        }

        new_id
    }

    async fn login(&self, credentials: Credentials) -> Result<Token> {
        let is_valid = self.credential_repo.check_credentials_db(&self.db, &credentials).await;

        match is_valid {
            Ok(true) => {
                let uuid = Uuid::new_v4().to_string();
                self.token_repo.save_token(&uuid, &credentials.username).await?;
                Ok(uuid)
            }
            Ok(false) => Err(Error::BadCredentials),
            Err(err) => Err(err),
        }
    }

    async fn authenticate(&self, token: Token) -> Result<String> {
        self.token_repo.get_username(&token).await
    }
}

#[cfg(test)]
mod tests;
