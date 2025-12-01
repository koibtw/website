use axum::{Json, extract::State, response::IntoResponse};

use crate::{ServerState, constants};

pub async fn start_handler(State(state): State<ServerState>) -> impl IntoResponse {
    Json(
        state
            .nixdle
            .start_game(format!("{}/api/nixdle/attempt", constants::HOST)),
    )
}

pub async fn attempt_handler(
    State(state): State<ServerState>,
    Json(data): Json<nixdle::api::AttemptData>,
) -> impl IntoResponse {
    Json(state.nixdle.attempt_game(&data.input, data.attempts))
}
