use crate::{config::context::AppContext, model::*, service::*, validation::*};
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use hyper::header::LOCATION;

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/authenticate", post(authenticate))
}

#[tracing::instrument(skip(ctx))]
async fn register(Valid(credentials): Valid<Credentials>, ctx: AppContext) -> Result<impl IntoResponse> {
    let service = AuthService::new(&ctx.db, &ctx.redis);
    let new_id: i64 = service.register(credentials).await?;

    // TODO create a /profile/<username> route that requires authentication
    let location = format!("/profile/{}", new_id);
    let headers = [(LOCATION, location)];
    Ok((StatusCode::CREATED, headers))
}

#[tracing::instrument(skip(ctx))]
async fn login(Valid(credentials): Valid<Credentials>, ctx: AppContext) -> Result<Json<LoginOk>> {
    let service = AuthService::new(&ctx.db, &ctx.redis);
    let token = service.login(credentials).await?;
    let response = Json(LoginOk { token });
    Ok(response)
}

#[tracing::instrument(skip(ctx))]
async fn authenticate(token: Token, ctx: AppContext) -> Result<Json<AuthOk>> {
    let service = AuthService::new(&ctx.db, &ctx.redis);
    let username = service.authenticate(token).await?;
    let response = Json(AuthOk { username });
    Ok(response)
}
