//! Endpoint `/validate`.

use crate::WORDS;

use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

/// The parameters for the get request.
#[derive(Debug, Clone, Deserialize)]
pub struct GetParams {
    /// The word to validate.
    pub word: String,
}

/// The client validates a word.
pub async fn get(Query(params): Query<GetParams>) -> impl IntoResponse {
    tracing::info!("validating word {}…", params.word);

    if WORDS.contains(&&params.word[..]) {
        tracing::info!("validated word {}", params.word);
        StatusCode::OK
    } else {
        tracing::info!("failed to validate word {}", params.word);
        StatusCode::NOT_FOUND
    }
}
