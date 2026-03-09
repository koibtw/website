use crate::DbPool;
use axum::{
  Json,
  extract::State,
  http::{HeaderMap, StatusCode},
  response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::chrono};
use std::time::Duration;

#[derive(Deserialize)]
pub struct Webhook {
  name: String,
  album: String,
  artist: String,
}

#[derive(Serialize, FromRow)]
struct MusicEntry {
  id: bool,
  name: String,
  album: String,
  artist: String,
  created_at: chrono::NaiveDateTime,
}

pub async fn start_handler(
  State(pool): State<DbPool>,
  headers: HeaderMap,
  Json(data): Json<Webhook>,
) -> Result<impl IntoResponse, impl IntoResponse> {
  if !verify_headers(&headers) {
    return Err((StatusCode::UNAUTHORIZED, "dont hack me bruh".to_string()));
  }

  match sqlx::query(
        "INSERT INTO music (name, album, artist) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, album = EXCLUDED.album, artist = EXCLUDED.artist",
    )
    .bind(&data.name)
    .bind(&data.album)
    .bind(&data.artist)
    .fetch_one(&pool)
    .await
    {
        Ok(_) => Ok((StatusCode::CREATED, "ok".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn stop_handler(
  State(pool): State<DbPool>,
  headers: HeaderMap,
) -> Result<impl IntoResponse, impl IntoResponse> {
  if !verify_headers(&headers) {
    return Err((StatusCode::UNAUTHORIZED, "dont hack me bruh".to_string()));
  }

  match sqlx::query("DELETE FROM music").fetch_one(&pool).await {
    Ok(_) => Ok((StatusCode::OK, "ok".to_string())),
    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
  }
}

pub async fn get_handler(
  State(pool): State<DbPool>,
) -> Result<impl IntoResponse, impl IntoResponse> {
  match sqlx::query_as::<_, MusicEntry>("SELECT * FROM music")
    .fetch_all(&pool)
    .await
  {
    Ok(data) => {
      if !data.is_empty()
        && data[0].created_at + Duration::new(30 * 60, 0) < chrono::Utc::now().naive_utc()
      {
        println!("some weirdness going on, clearing music table");
        match sqlx::query("DELETE FROM music").fetch_one(&pool).await {
          Ok(_) => return Ok((StatusCode::OK, Json(Vec::<MusicEntry>::new()))),
          Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
          }
        }
      }
      Ok((StatusCode::OK, Json(data)))
    }
    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
  }
}

fn verify_headers(headers: &HeaderMap) -> bool {
  if !headers.contains_key("Authorization")
    || headers["Authorization"] != format!("Basic {}", std::env::var("JELLYFIN_SECRET").unwrap())
  {
    println!("fucker tried to hack");
    false
  } else {
    true
  }
}
