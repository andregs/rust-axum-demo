use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
};
use hyper::body;
use serde_json::{json, Value};
use tower::ServiceExt;
use utils::config::{self, Config, Profile};
use uuid::Uuid;

// see more examples at https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

#[tokio::test]
async fn it_should_pass_e2e_happy_path() {
    let test_profile = Profile::const_new("test");
    let config = Config::load_for(test_profile).unwrap();
    let router = config::app::new_router(config).await.unwrap();

    let username = format!("test{}", Uuid::new_v4().as_simple());
    let req_json = json!({ "username": username, "password": "12345678" });
    let bytes = serde_json::to_vec(&req_json).unwrap();

    let request = Request::builder()
        .method(Method::POST)
        .uri("/register")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(bytes))
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let location = response
        .headers()
        .get("Location")
        .expect("location header was expected")
        .to_str()
        .expect("a valid header string was expected");

    assert!(location.starts_with("/profile/"));

    let new_id = location.split('/').last().expect("generated id was expected");
    let new_id = new_id.parse::<u64>().expect("numeric id was expected");
    assert!(new_id > 0);
    // let bytes = body::to_bytes(response.into_body()).await.unwrap();
    // let res_json: Value = serde_json::from_slice(&bytes).unwrap();
    // assert_eq!(req_json, res_json);
}
