use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::model::*;

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/authenticate", post(authenticate))
}

async fn register(Json(credentials): Json<Credentials>) -> impl IntoResponse {
    tracing::info!("Registering {:?}", credentials);
    (StatusCode::CREATED, Json(credentials))
}

async fn login(Json(credentials): Json<Credentials>) -> Json<LoginOk> {
    tracing::info!("Login: {:?}", credentials);
    Json(LoginOk { token: "token!".into() })
}

async fn authenticate(token: Token) -> Json<AuthOk> {
    tracing::info!("Authenticating: {:?}", token);
    Json(AuthOk {
        username: "user!".into(),
    })
}
