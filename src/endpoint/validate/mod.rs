//! Endpoint `/validate`.

use crate::WORDS;

use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Clone, Deserialize)]
pub struct Params {
    word: String,
}

pub async fn get(Query(params): Query<Params>) -> impl IntoResponse {
    info!("validating word {}…", params.word);

    if WORDS.contains(&&params.word[..]) {
        info!("validated word {}", params.word);
        (StatusCode::OK).into_response()
    } else {
        info!("failed to validate word {}", params.word);
        (StatusCode::NOT_FOUND).into_response()
    }
}
