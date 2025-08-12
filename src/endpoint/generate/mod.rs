//! Endpoint `/generate`.

use crate::{
    database::{
        POOL,
        puzzles::{get_puzzle, insert_or_update_solution},
        types::{Puzzle, PuzzleDate, PuzzleSolution},
    },
    endpoint::root::fetch_random_word,
};

use api_framework::framework::State;
use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PostParams {
    date: String,
}

pub async fn post(Query(params): Query<PostParams>) -> impl IntoResponse {
    let mut conn = match POOL.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    match PuzzleDate::new(&params.date) {
        Ok(date) => {
            if let Some(puzzle) = get_puzzle(&mut conn, &date) {
                // There is an existing puzzle
                (StatusCode::OK, Json(puzzle)).into_response()
            } else {
                // There isn't any existing puzzles
                if let Some(puzzle) = get_puzzle(&mut conn, &date) {
                    (StatusCode::OK, Json(puzzle)).into_response()
                } else {
                    let str = match fetch_random_word().await {
                        State::Success(str) => str,
                        _ => return (StatusCode::NOT_FOUND).into_response(),
                    };

                    match PuzzleSolution::new(&str) {
                        Ok(solution) => {
                            match insert_or_update_solution(&mut conn, &date, &solution) {
                                Ok(_) => (StatusCode::CREATED, Json(Puzzle::new(date, solution)))
                                    .into_response(),
                                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                                    .into_response(),
                            }
                        }
                        Err(err) => {
                            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                        }
                    }
                }
            }
        }
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
