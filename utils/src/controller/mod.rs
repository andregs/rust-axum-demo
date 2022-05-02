use crate::{config::app::AppContext, model::*, service::*, validation::*};
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use hyper::header::LOCATION;
use tracing::{debug, info};

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/authenticate", post(authenticate))
}

#[tracing::instrument(skip(ctx))]
async fn register(Valid(credentials): Valid<Credentials>, ctx: AppContext) -> Result<impl IntoResponse> {
    debug!("Registering a new user");

    let service = AuthService::new(&ctx.db, &ctx.redis);
    let new_id: i64 = service.register(credentials).await?;

    // TODO create a /profile/<username> route that requires authentication
    let location = format!("/profile/{}", new_id);
    let headers = [(LOCATION, location)];
    Ok((StatusCode::CREATED, headers))
}

#[tracing::instrument]
async fn login(Valid(credentials): Valid<Credentials>) -> Json<LoginOk> {
    info!("Login successful");
    Json(LoginOk { token: "token!".into() })
}

#[tracing::instrument]
async fn authenticate(_token: Token) -> Json<AuthOk> {
    info!("Authenticating user");
    Json(AuthOk {
        username: "user!".into(),
    })
}
