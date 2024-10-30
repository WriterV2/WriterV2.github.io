use std::collections::HashMap;
use std::env;

use anyhow::Context;
use axum::body::Bytes;
use axum::extract::{Multipart, Request};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use subtle::ConstantTimeEq;

use crate::db::story::Story;
use crate::db::ProductDatabaseHandler;
use crate::error::AppError;

use super::ApiContext;

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

    if expected_token
        .as_bytes()
        .ct_ne(token.as_bytes())
        .unwrap_u8()
        == 1
    {
        eprintln!("Warning: Unauthorized access attempt with invalid token");
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

pub async fn admin_healthcheck() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn upload_story(
    ctx: Extension<ApiContext>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut form: HashMap<String, Bytes> = HashMap::new();
    while let Some(field) = multipart.next_field().await? {
        if let Some(label) = field.name() {
            form.insert(label.to_string(), field.bytes().await?);
        }
    }

    let name = if let Some(name_value) = form.get("name") {
        String::from_utf8(name_value.to_vec()).with_context(|| "Failed to parse name")?
    } else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    let description = if let Some(description_value) = form.get("description") {
        String::from_utf8(description_value.to_vec())
            .with_context(|| "Failed to parse description")?
    } else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    let language = if let Some(language_value) = form.get("language") {
        String::from_utf8(language_value.to_vec()).with_context(|| "Failed to parse language")?
    } else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    let pdf = if let Some(pdf_value) = form.get("pdf") {
        pdf_value.to_vec()
    } else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    let epub = if let Some(epub_value) = form.get("epub") {
        epub_value.to_vec()
    } else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    let story_template = Story {
        id: 0,
        pid: 0,
        language,
        pdf,
        epub,
    };

    let story = story_template.post(&ctx.pool, name, description).await?;
    Ok((StatusCode::CREATED, Json(story)).into_response())
}
