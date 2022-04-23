use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
};
use axum_demo::config;
use hyper::body;
use serde_json::{json, Value};
use tower::ServiceExt;

// see more examples at https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

#[tokio::test]
async fn it_should_pass_e2e_happy_path() {
    let (_, router) = config::app::config_router();

    let req_json = json!({ "username": "foo", "password": "12345678" });
    let bytes = serde_json::to_vec(&req_json).unwrap();

    let request = Request::builder()
        .method(Method::POST)
        .uri("/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(bytes))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = body::to_bytes(response.into_body()).await.unwrap();
    let res_json: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(req_json, res_json);
}
