use axum::{
    extract::Path,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use controller_macro::{controller, route};

pub struct V1Controller;

#[controller(path = "/v1")]
impl V1Controller {
    #[route("GET", "/")]
    pub async fn root_handler<B>(request: Request<B>) -> impl IntoResponse {
        let user_agent = request.headers().get("trace_id");
        println!("User-Agent: {:?}", user_agent);

        axum::response::Response::builder()
            .status(StatusCode::OK)
            .body("Hello, World!".to_string())
            .unwrap()
    }

    #[route("POST", "/echo/:message")]
    pub async fn echo_handler<B>(
        Path(message): Path<String>,
        request: Request<B>,
    ) -> impl IntoResponse {
        let user_agent = request.headers().get("trace_id");
        println!("User-Agent: {:?}", user_agent);

        axum::response::Response::builder()
            .status(StatusCode::OK)
            .body(message)
            .unwrap()
    }

    #[route("GET", "/test")]
    pub async fn test_handler<B>(request: Request<B>) -> impl IntoResponse {
        let user_agent = request.headers().get("trace_id");
        println!("User-Agent: {:?}", user_agent);

        axum::response::Response::builder()
            .status(StatusCode::OK)
            .body("Testing...".to_string())
            .unwrap()
    }
}
