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
    HISTORY_MAX_TRIES, PuzzleDate, PuzzleSolution, SubmitHistory, SubmitWord,
    puzzles::Model as Puzzle,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct GetParams {
    date: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetResponse {
    remaining_tries: usize,
    is_dirty: bool,
    is_completed: bool,
    history: Vec<SubmitWord>,
}

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
                remaining_tries: history.remaining_tries(),
                is_dirty: history.is_dirty,
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
                        remaining_tries: HISTORY_MAX_TRIES,
                        is_dirty: false,
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
