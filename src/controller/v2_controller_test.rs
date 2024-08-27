use crate::controller::v2_controller::V2Controller;
use axum::{
    body::{to_bytes, Body},
    extract::Path,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};

fn get_request() -> Request<Body> {
    Request::builder()
        .header("trace_id", "test-trace-id")
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn test_root_handler() {
    let request = get_request();

    let response: Response<Body> = V2Controller::root_handler(request).await.into_response();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert_eq!(body, "Hello, World 2!");
}

#[tokio::test]
async fn test_echo_handler() {
    let request = get_request();

    let response: Response<Body> = V2Controller::echo_handler(Path("test".to_string()), request)
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert_eq!(body, "test");
}

#[tokio::test]
async fn test_test_handler() {
    let request = get_request();

    let response: Response<Body> = V2Controller::test_handler(request).await.into_response();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert_eq!(body, "Testing... 2");
}
