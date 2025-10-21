use axum::{body::Body, http::{Request, StatusCode}, Router};
use tower::ServiceExt; // for `app.oneshot()`
use axum::routing::get;
use hyper::body::to_bytes;

use controller::{
    AppState,
    root_handler,
    register_agent,
    get_agent_status,
    submit_task,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

async fn create_app() -> Router {
    let app_state = Arc::new(Mutex::new(AppState::default()));

    Router::new()
        .route("/", get(root_handler))
        .route("/agent/register", get(register_agent))
        .route("/agent/status", get(get_agent_status))
        .route("/task/submit", get(submit_task))
        .with_state(app_state)
}

#[tokio::test]
async fn test_root_handler() {
    let app = create_app().await;

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Hello, Controller!");
}

#[tokio::test]
async fn test_register_agent() {
    let app = create_app().await;

    let response = app
        .oneshot(Request::builder().uri("/agent/register").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Agent registered with ID:"));
}

#[tokio::test]
async fn test_get_agent_status() {
    let app = create_app().await;

    // First, register an agent
    let _ = app
        .oneshot(Request::builder().uri("/agent/register").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // Then, get agent status
    let response = app
        .oneshot(Request::builder().uri("/agent/status").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body()).await.unwrap();
    let agents: HashMap<String, controller::models::Agent> = serde_json::from_slice(&body).unwrap();
    assert!(!agents.is_empty());
}

#[tokio::test]
async fn test_submit_task() {
    let app = create_app().await;

    let response = app
        .oneshot(Request::builder().uri("/task/submit").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body()).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Task submitted with ID:"));
}