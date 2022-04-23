use axum::{http::StatusCode, routing::post, Json, Router};

use crate::{model::*, validation::*};

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/authenticate", post(authenticate))
}

async fn register(Valid(credentials): Valid<Credentials>) -> (StatusCode, Json<Credentials>) {
    tracing::info!("Registering {:?}", credentials);
    (StatusCode::CREATED, Json(credentials))
}

async fn login(Valid(credentials): Valid<Credentials>) -> Json<LoginOk> {
    tracing::info!("Login: {:?}", credentials);
    Json(LoginOk { token: "token!".into() })
}

async fn authenticate(token: Token) -> Json<AuthOk> {
    tracing::info!("Authenticating: {:?}", token);
    Json(AuthOk {
        username: "user!".into(),
    })
}
