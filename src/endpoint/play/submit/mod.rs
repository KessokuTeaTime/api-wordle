//! Endpoint `/play/submit`.

use crate::{
    WORDS,
    database::{self, tables::histories::submit_to_history},
    middleware::session::SessionToken,
};

use axum::{Extension, Json, extract::Query, http::StatusCode, response::IntoResponse};
use entity::{PuzzleDate, PuzzleSolution, SubmitWord};
use serde::{Deserialize, Serialize};

/// The parameters for the post request.
#[derive(Debug, Clone, Deserialize)]
pub struct PostParams {
    /// The date of the puzzle.
    pub date: String,
}

/// The payload for the post request.
#[derive(Debug, Clone, Deserialize)]
pub struct PostPayload {
    /// The answer being submitted.
    pub answer: String,
}

/// The response for the post request.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PostResponse {
    /// The number of letters in the word.
    pub letters_count: usize,
    /// The number of remaining tries.
    pub remaining_tries: usize,
    /// Whether the puzzle has been completed.
    pub is_completed: bool,
    /// The history of submitted words.
    pub history: Vec<SubmitWord>,
}

/// The client submits a word to solve the puzzle.
pub async fn post(
    session: Option<Extension<SessionToken>>,
    Query(params): Query<PostParams>,
    Json(payload): Json<PostPayload>,
) -> impl IntoResponse {
    let session = match session {
        Some(Extension(SessionToken(session))) => session,
        None => return (StatusCode::NOT_FOUND).into_response(),
    };

    let db = database::acquire_or_response!();

    let (date, answer) = match (
        PuzzleDate::try_from(&params.date[..]),
        PuzzleSolution::try_from(&payload.answer[..]),
    ) {
        (Ok(date), Ok(answer)) if WORDS.contains(&&answer.to_string()[..]) => (date, answer),
        _ => return (StatusCode::BAD_REQUEST).into_response(),
    };

    match submit_to_history(&db, &date, &session, &answer).await {
        Ok(result) => (
            StatusCode::ACCEPTED,
            Json(PostResponse {
                letters_count: result.submit_history.letters_count(),
                remaining_tries: result.submit_history.remaining_tries(),
                is_completed: result.is_completed,
                history: result.submit_history.into_vec(),
            }),
        )
            .into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
