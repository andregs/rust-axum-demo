use anyhow::anyhow;
use axum::{
    extract::rejection::JsonRejection,
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use hyper::{header::WWW_AUTHENTICATE, HeaderMap};
use serde::Serialize;
use std::borrow::Cow;
use tracing::{debug, Level};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Duplicated username.")]
    Duplicated(#[source] sqlx::Error),

    #[error("Username is too big.")]
    TooBig(#[source] sqlx::Error),

    #[error("Username and/or password mismatch.")]
    BadCredentials,

    // #[error("Token does not represent an authenticated user.")]
    // BadToken,
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),

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

impl IntoResponse for Error {
    #[tracing::instrument(level = Level::TRACE)]
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::Duplicated(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::TooBig(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::BadCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::ValidationError(errors) => (
                StatusCode::BAD_REQUEST,
                format!("{}", errors).replace(", ", " ").replace('\n', " "),
            ),
            Error::AxumJsonRejection(json_rejection) => match json_rejection {
                JsonRejection::JsonDataError(source) => (StatusCode::UNPROCESSABLE_ENTITY, extract_serde_message(source)),
                JsonRejection::JsonSyntaxError(source) => (StatusCode::BAD_REQUEST, extract_serde_message(source)),
                JsonRejection::MissingJsonContentType(source) => (StatusCode::UNSUPPORTED_MEDIA_TYPE, format!("{}", source)),
                _ => (StatusCode::BAD_REQUEST, "Unknown Reason".to_string()),
            },
            Error::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let headers = if status == StatusCode::UNAUTHORIZED {
            [(WWW_AUTHENTICATE, HeaderValue::from_static("Token"))]
                .into_iter()
                .collect::<HeaderMap>()
        } else {
            HeaderMap::new()
        };

        let id = Uuid::new_v4().to_string();
        let error = ErrorBody { id, message };
        debug!("{:?}", error);

        (status, headers, Json(error)).into_response()
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorBody {
    id: String,
    message: String,
}

/// see https://docs.rs/axum/0.5.3/axum/extract/index.html#accessing-inner-errors
fn extract_serde_message<E>(err: E) -> String
where
    E: std::error::Error + 'static,
{
    if let Some(serde_json_err) = find_error_source::<serde_json::Error>(&err) {
        format!("{}", serde_json_err)
    } else {
        "Unknown error".to_string()
    }
}

fn find_error_source<'a, T>(err: &'a (dyn std::error::Error + 'static)) -> Option<&'a T>
where
    T: std::error::Error + 'static,
{
    if let Some(err) = err.downcast_ref::<T>() {
        Some(err)
    } else if let Some(source) = err.source() {
        find_error_source(source)
    } else {
        None
    }
}
