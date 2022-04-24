use crate::{model::*, validation::*};
use axum::{http::StatusCode, routing::post, Json, Router};

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/authenticate", post(authenticate))
}

#[tracing::instrument]
async fn register(Valid(credentials): Valid<Credentials>) -> (StatusCode, Json<Credentials>) {
    tracing::debug!("Registering a new user");
    (StatusCode::CREATED, Json(credentials))
}

#[tracing::instrument]
async fn login(Valid(credentials): Valid<Credentials>) -> Json<LoginOk> {
    tracing::info!("Login successful");
    Json(LoginOk { token: "token!".into() })
}

#[tracing::instrument]
async fn authenticate(_token: Token) -> Json<AuthOk> {
    tracing::info!("Authenticating user");
    Json(AuthOk {
        username: "user!".into(),
    })
}
