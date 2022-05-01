use crate::model::Error;
use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, RequestParts},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Default)]
pub struct Valid<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for Valid<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await?;
        value.validate()?;
        Ok(Valid(value))
    }
}
