//! Endpoint root.

use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use chrono::Datelike as _;
use entity::puzzles::Model as Puzzle;
use entity::{PuzzleDate, PuzzleSolution, puzzles::ResultPuzzle};
use serde::{Deserialize, Serialize};

use crate::database::{
    self,
    tables::puzzles::{get_puzzle, get_puzzles, insert_solution},
};

/// The parameters for the get request.
#[derive(Debug, Clone, Deserialize)]
pub struct GetParams {
    /// The date of the puzzle to get.
    pub date: Option<String>,
    /// Whether to generate a new puzzle if missing.
    pub generate_if_missing: Option<bool>,
}

/// The response for a single puzzle get request.
#[derive(Debug, Clone, Serialize)]
pub struct GetResponsePuzzle(ResultPuzzle);

/// The response for multiple puzzles get request.
#[derive(Debug, Clone, Serialize)]
pub struct GetResponsePuzzles {
    /// The number of available puzzles.
    pub count: usize,
    /// The puzzles.
    pub puzzles: Vec<ResultPuzzle>,
}

/// The client gets puzzle information.
///
/// # Panics
///
/// Panics if cannot get a random word from [`random_word`].
pub async fn get(Query(params): Query<GetParams>) -> impl IntoResponse {
    let db = database::acquire_or_response!();

    if let Some(date) = params.date {
        let date = match PuzzleDate::try_from(&date[..]) {
            Ok(date) => date,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };

        if let Some(puzzle) = get_puzzle(&db, &date).await {
            (
                StatusCode::OK,
                Json(GetResponsePuzzle(puzzle.to_result_puzzle())),
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
    } else {
        let puzzles: Vec<ResultPuzzle> = get_puzzles(&db)
            .await
            .into_iter()
            .map(Puzzle::to_result_puzzle)
            .collect();
        (
            StatusCode::OK,
            Json(GetResponsePuzzles {
                count: puzzles.len(),
                puzzles,
            }),
        )
            .into_response()
    }
}

/// The parameters for the post request.
#[derive(Debug, Clone, Deserialize)]
pub struct PostParams {
    /// Whether to ignore conflict if the puzzle already exists.
    pub ignores_conflict: Option<bool>,
}

/// The payload for the post request.
#[derive(Debug, Clone, Deserialize)]
pub struct PostPayload {
    /// The date of the puzzle.
    pub date: String,
    /// The solution of the puzzle.
    pub solution: String,
}

/// The client posted a puzzle.
pub async fn post(
    Query(params): Query<PostParams>,
    Json(payload): Json<PostPayload>,
) -> impl IntoResponse {
    let db = database::acquire_or_response!();

    let (date, solution) = match (
        PuzzleDate::try_from(&payload.date[..]),
        PuzzleSolution::try_from(&payload.solution[..]),
    ) {
        (Ok(date), Ok(solution)) => (date, solution),
        _ => return (StatusCode::BAD_REQUEST).into_response(),
    };

    if !params.ignores_conflict.unwrap_or(false) && get_puzzle(&db, &date).await.is_some() {
        // there is an existing puzzle and we shouldn't proceed
        (StatusCode::CONFLICT).into_response()
    } else {
        // there isn't any existing puzzles
        match insert_solution(&db, &date, &solution).await {
            Ok(_) => (StatusCode::CREATED).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    }
}
