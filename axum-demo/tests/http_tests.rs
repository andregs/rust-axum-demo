use anyhow::{Context, Result};
use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
};
use axum_demo::{
    config::{context, Config, Profile},
    model::{AuthOk, LoginOk},
};
use hyper::{body, Client};
use serde_json::{json, Value};
use std::net::{SocketAddr, TcpListener};
use uuid::{Uuid, Variant::RFC4122};

// see more examples at https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

#[tokio::test]
async fn it_should_pass_e2e_happy_path() -> Result<()> {
    let address = start_server().await?;
    let address = address.as_str();
    let client = Client::new();

    // registration

    let username = format!("test{}", Uuid::new_v4().as_simple());
    let body = &json!({ "username": username, "password": "12345678" });
    let response = client.request(post_json(address, "/register", body)?).await?;
    assert_eq!(response.status(), StatusCode::CREATED);

    let location = response
        .headers()
        .get(header::LOCATION)
        .context("should get location header")?
        .to_str()?;

    assert!(location.starts_with("/profile/"));

    let new_id = location.split('/').last().context("should get generated id")?;
    let new_id = new_id.parse::<u64>()?;
    assert!(new_id > 0);

    // login

    let response = client.request(post_json(address, "/login", body)?).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = body::to_bytes(response.into_body()).await?;
    let actual: LoginOk = serde_json::from_slice(&bytes)?;
    let token = Uuid::parse_str(&actual.token)?;
    assert_eq!(token.get_variant(), RFC4122);

    // authentication

    let body = token.to_string();
    let response = client.request(post_plain(address, "/authenticate", body)?).await?;
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = body::to_bytes(response.into_body()).await?;
    let actual: AuthOk = serde_json::from_slice(&bytes)?;
    assert_eq!(username, actual.username);

    Ok(())
}

// aux -----

async fn start_server() -> Result<String> {
    let test_profile = Profile::const_new("test");
    let config = Config::load_for(test_profile)?;
    let router = context::new_router(config).await?;
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap())?;
    let address = listener.local_addr()?;

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });

    Ok(format!("http://{}", address))
}

fn post_json(base: &str, path: &str, json: &Value) -> Result<Request<Body>> {
    let bytes = serde_json::to_vec(json)?;
    let body = Body::from(bytes);
    let req = Request::builder()
        .method(Method::POST)
        .uri(format!("{base}{path}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(body)?;

    Ok(req)
}

fn post_plain(base: &str, path: &str, body: String) -> Result<Request<Body>> {
    let body = Body::from(body);
    let req = Request::builder()
        .method(Method::POST)
        .uri(format!("{base}{path}"))
        .header(header::CONTENT_TYPE, "text/plain")
        .body(body)?;

    Ok(req)
}
