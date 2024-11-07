use std::collections::HashMap;
use std::io::{BufWriter, Write};

use anyhow::Context;
use axum::body::Bytes;
use axum::extract::{Multipart, Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use rand::Rng;
use subtle::ConstantTimeEq;

use crate::db::story::{format_filepath, Story};
use crate::db::ProductDatabaseHandler;
use crate::error::AppError;

use super::{ApiContext, AppState};

pub fn generate_token() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    let token: String = (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("New Token generated: {}", token);
    token
}

pub async fn admin_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut expected_token = state.admin_token.lock().await;

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
        *expected_token = generate_token();
        return Err(StatusCode::UNAUTHORIZED);
    }
    *expected_token = generate_token();

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
        let filename = format_filepath(&name, "pdf");
        let file = std::fs::File::create_new(filename)?;
        BufWriter::new(file).write_all(pdf_value)?;
        pdf_value.to_vec()
    } else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    let epub = if let Some(epub_value) = form.get("epub") {
        let filename = format_filepath(&name, "epub");
        let file = std::fs::File::create_new(filename)?;
        BufWriter::new(file).write_all(epub_value)?;
        epub_value.to_vec()
    } else {
        return Ok(StatusCode::BAD_REQUEST.into_response());
    };

    let mut tags: Vec<String> = Vec::new();
    if let Some(tags_value) = form.get("tag") {
        for tag in String::from_utf8(tags_value.to_vec())?.split(",") {
            tags.push(tag.trim().to_string());
        }
    }

    let story_template = Story {
        id: 0,
        pid: 0,
        language,
        pdf,
        epub,
    };

    let story = story_template
        .post(&ctx.pool, name, description, tags)
        .await?;
    Ok((StatusCode::CREATED, Json(story)).into_response())
}

pub async fn delete_story(
    ctx: Extension<ApiContext>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    if let Some(field) = multipart.next_field().await? {
        if let Some(label) = field.name() {
            if label == "id" {
                let pid = field.text().await?.parse::<i64>()?;
                Story::delete(&ctx.pool, pid).await?;
                return Ok((StatusCode::NO_CONTENT).into_response());
            }
            return Ok(StatusCode::BAD_REQUEST.into_response());
        }
        return Ok(StatusCode::BAD_REQUEST.into_response());
    }
    Ok(StatusCode::BAD_REQUEST.into_response())
}
