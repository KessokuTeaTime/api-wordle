//! Endpoint `/dates`.

use axum::{Json, http::StatusCode, response::IntoResponse};
use entity::PuzzleDate;

use crate::database::{self, tables::puzzles::get_dates};

pub async fn get() -> impl IntoResponse {
    let db = database::acquire_or_response!();

    let dates: Vec<String> = get_dates(&db, false)
        .await
        .iter()
        .map(PuzzleDate::to_string)
        .collect();
    (StatusCode::OK, Json(dates)).into_response()
}
