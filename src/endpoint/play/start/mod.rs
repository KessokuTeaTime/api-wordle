//! Endpoint `/play/start`.

use crate::{
    database::{
        self,
        tables::{
            histories::{create_history, get_history},
            puzzles::{get_puzzle, insert_solution},
            sessions::insert_or_update_session,
        },
    },
    middleware::session::SessionToken,
};

use axum::{Extension, Json, extract::Query, http::StatusCode, response::IntoResponse};
use entity::{
    HISTORY_MAX_TRIES, PUZZLE_LETTERS_COUNT, PuzzleDate, PuzzleSolution, SubmitHistory, SubmitWord,
    puzzles::Model as Puzzle,
};
use serde::{Deserialize, Serialize};

/// The parameters for the get request.
#[derive(Debug, Clone, Deserialize)]
pub struct GetParams {
    /// The date of the puzzle in `YYYY-MM-DD` format.
    pub date: String,
}

/// The response for the get request.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetResponse {
    /// The number of letters in the word.
    pub letters_count: usize,
    /// The number of remaining tries.
    pub remaining_tries: usize,
    /// Whether the puzzle has been completed.
    pub is_completed: bool,
    /// The history of submitted words.
    pub history: Vec<SubmitWord>,
}

/// The client requests to start a puzzle session.
///
/// # Panics
///
/// Panics if cannot get a random word from [`random_word`].
pub async fn get(
    session: Option<Extension<SessionToken>>,
    Query(params): Query<GetParams>,
) -> impl IntoResponse {
    let session = match session {
        Some(Extension(SessionToken(session))) => session,
        None => return (StatusCode::NOT_FOUND).into_response(),
    };

    let db = database::acquire_or_response!();

    let date = match PuzzleDate::try_from(&params.date[..]) {
        Ok(date) => date,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };

    match insert_or_update_session(&db, &session).await {
        Ok(_) => {}
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }

    match get_history(&db, &date, &session).await {
        Some(history) => (
            StatusCode::OK,
            Json(GetResponse {
                letters_count: history.letters_count(),
                remaining_tries: history.remaining_tries(),
                is_completed: history.is_completed,
                history: history
                    .submit_history
                    .map(SubmitHistory::into_vec)
                    .unwrap_or_default(),
            }),
        )
            .into_response(),
        None => {
            let solution = match get_puzzle(&db, &date).await {
                Some(Puzzle { solution, .. }) => solution,
                None => {
                    let str = random_word::get_len(5, random_word::Lang::En).unwrap();
                    let solution = match PuzzleSolution::try_from(str) {
                        Ok(solution) => solution,
                        Err(err) => {
                            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                                .into_response();
                        }
                    };

                    match insert_solution(&db, &date, &solution).await {
                        Ok(_) => solution,
                        Err(err) => {
                            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                                .into_response();
                        }
                    }
                }
            };

            match create_history(&db, &date, &session, &solution).await {
                Ok(_) => (
                    StatusCode::CREATED,
                    Json(GetResponse {
                        letters_count: PUZZLE_LETTERS_COUNT,
                        remaining_tries: HISTORY_MAX_TRIES,
                        is_completed: false,
                        ..Default::default()
                    }),
                )
                    .into_response(),
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
            }
        }
    }
}
