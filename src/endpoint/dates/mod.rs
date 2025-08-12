//! Endpoint `/dates`.

use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::database::{POOL, puzzles::get_dates, types::PuzzleDate};

pub async fn get() -> impl IntoResponse {
    let mut conn = match POOL.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let dates: Vec<String> = get_dates(&mut conn)
        .iter()
        .map(PuzzleDate::to_string)
        .collect();
    (StatusCode::OK, Json(dates)).into_response()
}
