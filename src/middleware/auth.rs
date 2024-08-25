use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response<Body>, StatusCode> {
    // req.headers()
    //     .get("authorization")
    //     .ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(next.run(req).await)
}
