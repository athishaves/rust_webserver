use crate::middleware::{auth::auth_middleware, context::context_middleware};
use axum::{
    extract::Path,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use controller_macro::{controller, route};

pub struct V2Controller;

#[controller(
    path = "/v2",
    middleware = "auth_middleware",
    middleware = "context_middleware"
)]
impl V2Controller {
    #[route("GET", "/")]
    pub async fn root_handler<B>(request: Request<B>) -> impl IntoResponse {
        let trace_id = request.headers().get("trace_id");
        println!("Trace ID: {:?}", trace_id);

        axum::response::Response::builder()
            .status(StatusCode::OK)
            .body("Hello, World 2!".to_string())
            .unwrap()
    }

    #[route("POST", "/echo/:message")]
    pub async fn echo_handler<B>(
        Path(message): Path<String>,
        request: Request<B>,
    ) -> impl IntoResponse {
        let trace_id = request.headers().get("trace_id");
        println!("Trace ID: {:?}", trace_id);

        axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(message)
            .unwrap()
    }

    #[route("GET", "/test")]
    pub async fn test_handler<B>(request: Request<B>) -> impl IntoResponse {
        let trace_id = request.headers().get("trace_id");
        println!("Trace ID: {:?}", trace_id);

        axum::response::Response::builder()
            .status(StatusCode::OK)
            .body("Testing... 2".to_string())
            .unwrap()
    }
}
