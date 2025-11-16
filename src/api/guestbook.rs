use super::{censor_input, ntfy_send, validate_input};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, types::chrono};

#[derive(Deserialize)]
pub struct Entry {
    name: String,
    website: Option<String>,
    message: String,
}

#[derive(Serialize, FromRow)]
struct GuestbookEntry {
    id: i32,
    name: String,
    website: Option<String>,
    message: String,
    created_at: chrono::NaiveDateTime,
}

pub async fn add_handler(
    State(pool): State<PgPool>,
    Json(data): Json<Entry>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let name = data.name.trim();
    let website = data.website.unwrap_or_default().trim().to_string();
    let message = data.message.trim();

    println!(
        "got new guestbook entry from {}{}:\n{}",
        name,
        if website.is_empty() {
            "".to_string()
        } else {
            format!(" ({})", website)
        },
        message
    );

    validate_input(name).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    validate_input(&website).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    validate_input(message).map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    if !website.is_empty()
        && (!website.starts_with("https://") || !website.contains('.') || website.contains(' '))
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "website has to be a valid URL starting with https://".to_string(),
        ));
    }

    if name.is_empty() || name.len() > 20 {
        return Err((
            StatusCode::BAD_REQUEST,
            "name has to be between 1 and 50 chars".to_string(),
        ));
    }
    if !website.is_empty() && website.len() > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            "website has to be between 0 and 100 chars".to_string(),
        ));
    }
    if message.is_empty() || message.len() > 150 {
        return Err((
            StatusCode::BAD_REQUEST,
            "message has to be between 1 and 150 chars".to_string(),
        ));
    }

    let name_censored = censor_input(name).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let message_censored = censor_input(message).map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    match sqlx::query_as::<_, GuestbookEntry>(
        "INSERT INTO guestbook (name, website, message) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(name_censored)
    .bind(website)
    .bind(message_censored)
    .fetch_one(&pool)
    .await
    {
        Ok(entry) => {
            ntfy_send(
                entry.name.clone(),
                if let Some(website) = &entry.website {
                    format!("{}\n{}", entry.message, website)
                } else {
                    entry.message.clone()
                },
            )
            .await;

            Ok((StatusCode::CREATED, Json(entry)))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_all_handler(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, GuestbookEntry>("SELECT * FROM guestbook ORDER BY id ASC")
        .fetch_all(&pool)
        .await
    {
        Ok(entries) => Ok((StatusCode::OK, Json(entries))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
