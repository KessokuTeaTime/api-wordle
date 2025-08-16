//! Endpoint `/dates`.

use crate::database::{self, tables::puzzles::get_dates};

use axum::{Json, http::StatusCode, response::IntoResponse};
use entity::PuzzleDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GetResponse {
    count: usize,
    dates: Vec<PuzzleDate>,
}

pub async fn get() -> impl IntoResponse {
    let db = database::acquire_or_response!();

    let dates: Vec<PuzzleDate> = get_dates(&db, false).await;
    (
        StatusCode::OK,
        Json(GetResponse {
            count: dates.len(),
            dates,
        }),
    )
        .into_response()
}
