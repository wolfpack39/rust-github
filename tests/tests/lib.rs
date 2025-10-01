use std::sync::Arc;
use gist_api::{api::handlers::gist_handler, application::state::AppState};
use tower::ServiceExt;
use http_body_util::BodyExt;

use axum::{body::Body, http::{Request, StatusCode}, response::{Response}, routing::get, Router};
use reqwest::Client;

// 
pub async fn app_setup() -> Router {
    let app_state = Arc::new(AppState {
        github_client: Client::new()
    });

    Router::new()
        .route("/", get(gist_handler::get_user_gists))
        .with_state(app_state)
}

// helper for making the request to the test router
async fn make_request(app: &Router, path: &str) -> Response {
    app.clone()
        .oneshot(
            Request::builder()
            .uri(path)
            .body(Body::empty())
            .unwrap()
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn test_gists_success() {
    let app = app_setup().await;
    let response = make_request(&app, "/octocat").await;

    // Inspect http response status for success
    assert_eq!(response.status(), StatusCode::OK, "Expected status code 200 for octocat");

    // Check response body
    let response_bytes = response
        .into_body()   
        .collect()
        .await
        .unwrap()
        .to_bytes();

    let response_str = String::from_utf8(response_bytes.to_vec()).unwrap();
    assert!(response_str.starts_with('[') && response_str.ends_with(']'), "Response body is not a JSON array");
}