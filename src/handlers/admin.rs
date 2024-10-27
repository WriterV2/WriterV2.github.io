use std::env;

use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use subtle::ConstantTimeEq;

pub async fn admin_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let expected_token = env::var("TOKEN").map_err(|_| {
        eprintln!("Error: TOKEN environment variable is not set");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let token = headers
        .get("X-Admin-Token")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !expected_token
        .as_bytes()
        .ct_eq(token.as_bytes())
        .unwrap_u8()
        == 1
    {
        eprintln!("Warning: Unauthorized access attempt with invalid token");
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

pub async fn admin_handler() -> &'static str {
    "Hello World"
}
