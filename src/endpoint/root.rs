//! Endpoint root.

use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use chrono::Datelike;
use entity::puzzles::Model as Puzzle;
use entity::{PuzzleDate, PuzzleSolution, puzzles::ResultPuzzle};
use serde::{Deserialize, Serialize};

use crate::database::tables::puzzles::{delete_solution, update_solution};
use crate::database::{
    self,
    tables::puzzles::{get_puzzle, get_puzzles, insert_solution},
};

#[derive(Debug, Clone, Deserialize)]
pub struct GetParams {
    date: Option<String>,
    generate_if_missing: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetResponsePuzzle(ResultPuzzle);

#[derive(Debug, Clone, Serialize)]
pub struct GetResponsePuzzles {
    count: usize,
    puzzles: Vec<ResultPuzzle>,
}

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
        let puzzles: Vec<ResultPuzzle> = get_puzzles(&db, false)
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

#[derive(Debug, Clone, Deserialize)]
pub struct PostParams {
    ignores_conflict: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPayload {
    date: String,
    solution: String,
}

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
        // There is an existing puzzle and we shouldn't proceed
        (StatusCode::CONFLICT).into_response()
    } else {
        // There isn't any existing puzzles
        match insert_solution(&db, &date, &solution).await {
            Ok(_) => (StatusCode::CREATED).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutPayload {
    date: String,
    solution: String,
}

pub async fn put(Json(payload): Json<PutPayload>) -> impl IntoResponse {
    let db = database::acquire_or_response!();

    let (date, solution) = match (
        PuzzleDate::try_from(&payload.date[..]),
        PuzzleSolution::try_from(&payload.solution[..]),
    ) {
        (Ok(date), Ok(solution)) => (date, solution),
        _ => return (StatusCode::BAD_REQUEST).into_response(),
    };

    match update_solution(&db, &date, &solution).await {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(_) => (StatusCode::NOT_FOUND).into_response(),
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteParams {
    date: String,
}

pub async fn delete(Query(params): Query<DeleteParams>) -> impl IntoResponse {
    let db = database::acquire_or_response!();

    let date = match PuzzleDate::try_from(&params.date[..]) {
        Ok(date) => date,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };

    if get_puzzle(&db, &date).await.is_some() {
        // There is an existing puzzle
        match delete_solution(&db, &date).await {
            Ok(_) => (StatusCode::NO_CONTENT).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    } else {
        // There isn't any existing puzzles
        (StatusCode::NOT_FOUND).into_response()
    }
}
