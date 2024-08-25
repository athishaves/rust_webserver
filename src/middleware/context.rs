use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};
use uuid::Uuid;

pub async fn context_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let trace_id = Uuid::new_v4().to_string();
    req.headers_mut()
        .append("trace_id", trace_id.parse().unwrap());
    Ok(next.run(req).await)
}
