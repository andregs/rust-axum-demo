use std::error::Error;

use axum::{
    async_trait,
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest, RequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
    BoxError, Json,
};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, Level};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct Valid<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for Valid<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = InvalidRequest;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await?;
        value.validate()?;
        Ok(Valid(value))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidRequest {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

#[derive(Serialize, Debug)]
pub struct ErrorBody {
    id: String,
    message: String,
}

impl IntoResponse for InvalidRequest {
    #[tracing::instrument(level = Level::TRACE)]
    fn into_response(self) -> Response {
        let (status, message) = match self {
            InvalidRequest::ValidationError(errors) => (
                StatusCode::BAD_REQUEST,
                format!("{}", errors).replace(", ", " ").replace('\n', " "),
            ),
            InvalidRequest::AxumJsonRejection(json_rejection) => match json_rejection {
                JsonRejection::JsonDataError(source) => (StatusCode::UNPROCESSABLE_ENTITY, extract_serde_message(source)),
                JsonRejection::JsonSyntaxError(source) => (StatusCode::BAD_REQUEST, extract_serde_message(source)),
                JsonRejection::MissingJsonContentType(source) => (StatusCode::UNSUPPORTED_MEDIA_TYPE, format!("{}", source)),
                _ => (StatusCode::BAD_REQUEST, "Unknown Reason".to_string()),
            },
        };

        let id = Uuid::new_v4().to_string();
        let error = ErrorBody { id, message };
        debug!("{:?}", error);

        (status, Json(error)).into_response()
    }
}

/// see https://docs.rs/axum/0.5.3/axum/extract/index.html#accessing-inner-errors
fn extract_serde_message<E>(err: E) -> String
where
    E: Error + 'static,
{
    if let Some(serde_json_err) = find_error_source::<serde_json::Error>(&err) {
        format!("{}", serde_json_err)
    } else {
        "Unknown error".to_string()
    }
}

fn find_error_source<'a, T>(err: &'a (dyn Error + 'static)) -> Option<&'a T>
where
    T: Error + 'static,
{
    if let Some(err) = err.downcast_ref::<T>() {
        Some(err)
    } else if let Some(source) = err.source() {
        find_error_source(source)
    } else {
        None
    }
}
