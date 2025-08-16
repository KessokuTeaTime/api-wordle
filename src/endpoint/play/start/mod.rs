//! Endpoint `/play/start`.

use crate::{
    database::{self, tables::puzzles::get_puzzle},
    middleware::session::SessionToken,
};

use axum::{Extension, Json, extract::Query, http::StatusCode, response::IntoResponse};
use entity::{HISTORY_MAX_TRIES, PuzzleDate, puzzles::Model as Puzzle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct GetParams {
    date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetResponse {
    tries: usize,
}

pub async fn get(
    token: Option<Extension<SessionToken>>,
    Query(params): Query<GetParams>,
) -> impl IntoResponse {
    if token.is_none() {
        return (StatusCode::NOT_FOUND).into_response();
    }

    let db = database::acquire_or_response!();

    let date = match PuzzleDate::try_from(&params.date[..]) {
        Ok(date) => date,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };

    if let Some(Puzzle { solution, .. }) = get_puzzle(&db, &date).await {
        (
            StatusCode::OK,
            Json(GetResponse {
                tries: HISTORY_MAX_TRIES,
            }),
        )
            .into_response()
    } else if params.generate_if_missing.unwrap_or(false) {
        let str = random_word::get_len(5, random_word::Lang::En).unwrap();
        let solution = match PuzzleSolution::try_from(str) {
            Ok(solution) => solution,
            Err(err) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
            }
        };

        match insert_solution(&db, &date, &solution).await {
            Ok(_) => {
                if date.inner().year() == 2077 {
                    (
                        StatusCode::CREATED,
                        [("x-greeting", "Good morning, Night City!")],
                        Json(GetResponsePuzzle(ResultPuzzle { date, solution })),
                    )
                        .into_response()
                } else {
                    (
                        StatusCode::CREATED,
                        Json(GetResponsePuzzle(ResultPuzzle { date, solution })),
                    )
                        .into_response()
                }
            }
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}
