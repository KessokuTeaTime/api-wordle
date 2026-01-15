//! Endpoint `/dates`.

use crate::database::{self, tables::puzzles::get_dates};

use axum::{Json, http::StatusCode, response::IntoResponse};
use entity::PuzzleDate;
use serde::Serialize;

/// The response for the get request.
#[derive(Debug, Clone, Serialize)]
pub struct GetResponse {
    /// The total count of available dates.
    pub count: usize,
    /// The available puzzle dates.
    pub dates: Vec<PuzzleDate>,
}

/// The client gets the available puzzle dates.
pub async fn get() -> impl IntoResponse {
    let db = database::acquire_or_response!();

    let dates: Vec<PuzzleDate> = get_dates(&db).await;
    (
        StatusCode::OK,
        Json(GetResponse {
            count: dates.len(),
            dates,
        }),
    )
        .into_response()
}
