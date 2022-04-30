use crate::model::*;
use anyhow::anyhow;
use axum::async_trait;
use sqlx::{Executor, Pool, Postgres};
use std::borrow::Cow;

pub type Connection = Pool<Postgres>;
pub type Transaction = sqlx::Transaction<'static, Postgres>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CredentialRepoApi {
    async fn insert_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> Result<i64>;

    // I could make a generic version of check_credentials to avoid this duplication,
    // but I don't know how to make it work with automock.
    async fn check_credentials_db(&self, db: &Connection, credentials: &Credentials) -> Result<bool>;
    async fn check_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> Result<bool>;
}

pub struct PostgresCredentialRepo;

impl PostgresCredentialRepo {
    async fn insert_credentials<'ex, EX>(&self, executor: EX, credentials: &Credentials) -> Result<i64>
    where
        EX: 'ex + Executor<'ex, Database = Postgres>,
    {
        // sqlx validates query strings on compile-time
        sqlx::query!(
            r#"INSERT INTO credentials (username, password)
            VALUES ($1, crypt($2, gen_salt('bf')))
            RETURNING id"#,
            credentials.username,
            credentials.password,
        )
        .fetch_one(executor)
        .await
        .map(|row| row.id)
        .map_err(|err| err.into())
    }

    async fn check_credentials<'ex, EX>(&self, executor: EX, credentials: &Credentials) -> Result<bool>
    where
        EX: 'ex + Executor<'ex, Database = Postgres>,
    {
        sqlx::query_scalar!(
            // column name is special sqlx syntax to override the inferred type, check query! macro docs
            r#"SELECT password = crypt($1, password) as "not_null!"
            FROM credentials 
            WHERE username = $2"#,
            credentials.password,
            credentials.username,
        )
        .fetch_optional(executor)
        .await
        .map(|option| option.or(Some(false)).unwrap())
        .map_err(|err| err.into())
    }
}

#[async_trait]
impl CredentialRepoApi for PostgresCredentialRepo {
    async fn insert_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> Result<i64> {
        self.insert_credentials(tx, credentials).await
    }

    async fn check_credentials_db(&self, db: &Connection, credentials: &Credentials) -> Result<bool> {
        self.check_credentials(db, credentials).await
    }

    async fn check_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> Result<bool> {
        self.check_credentials(tx, credentials).await
    }
}

#[cfg(test)]
mod tests;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Duplicated username.")]
    Duplicated(#[source] sqlx::Error),

    #[error("Username is too big.")]
    TooBig(#[source] sqlx::Error),

    #[error("Sorry, we failed.")]
    Other(#[from] anyhow::Error),
}

impl From<sqlx::Error> for Error {
    fn from(source: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref err) = source {
            // https://www.postgresql.org/docs/current/errcodes-appendix.html
            if err.code() == Some(Cow::from("23505")) {
                return Error::Duplicated(source);
            } else if err.code() == Some(Cow::from("22001")) {
                return Error::TooBig(source);
            }
        }

        Error::Other(anyhow!(source))
    }
}
