//! Endpoint root.

use crate::database::{
    POOL,
    puzzles::{
        WORDS, delete_solution, get_puzzle, get_puzzles, insert_or_update_solution,
        insert_solution, update_solution,
    },
    types::{Puzzle, PuzzleDate, PuzzleSolution, ResultPuzzle},
};

use api_framework::framework::State;
use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use chrono::Datelike;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GetParams {
    date: Option<String>,
    generate_if_missing: Option<bool>,
}

pub async fn get(Query(params): Query<GetParams>) -> impl IntoResponse {
    let mut conn = match POOL.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    if let Some(date) = params.date {
        let date = match PuzzleDate::new(&date) {
            Ok(date) => date,
            Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        };

        if let Some(puzzle) = get_puzzle(&mut conn, &date) {
            (StatusCode::OK, Json(puzzle.to_result_puzzle())).into_response()
        } else if params.generate_if_missing.unwrap_or(false) {
            let str = random_word::get_len(5, random_word::Lang::En).unwrap();
            let solution = match PuzzleSolution::new(str) {
                Ok(solution) => solution,
                Err(err) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
                }
            };

            match insert_or_update_solution(&mut conn, &date, &solution) {
                Ok(_) => {
                    if date.inner().year() == 2077 {
                        (
                            StatusCode::CREATED,
                            [("x-greeting", "Good morning, Night City!")],
                            Json(Puzzle::new(date, solution).to_result_puzzle()),
                        )
                            .into_response()
                    } else {
                        (
                            StatusCode::CREATED,
                            Json(Puzzle::new(date, solution).to_result_puzzle()),
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
        let puzzles: Vec<ResultPuzzle> = get_puzzles(&mut conn, false)
            .into_iter()
            .map(Puzzle::to_result_puzzle)
            .collect();
        (StatusCode::OK, Json(puzzles)).into_response()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostPayload {
    date: String,
    solution: String,
}

pub async fn post(Json(payload): Json<PostPayload>) -> impl IntoResponse {
    let mut conn = match POOL.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let (date, solution) = match (
        PuzzleDate::new(&payload.date),
        PuzzleSolution::new(&payload.solution),
    ) {
        (Ok(date), Ok(solution)) => (date, solution),
        _ => return (StatusCode::BAD_REQUEST).into_response(),
    };

    match insert_solution(&mut conn, &date, &solution) {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(_) => (StatusCode::CONFLICT).into_response(),
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutPayload {
    date: String,
    solution: String,
}

pub async fn put(Json(payload): Json<PutPayload>) -> impl IntoResponse {
    let mut conn = match POOL.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let (date, solution) = match (
        PuzzleDate::new(&payload.date),
        PuzzleSolution::new(&payload.solution),
    ) {
        (Ok(date), Ok(solution)) => (date, solution),
        _ => return (StatusCode::BAD_REQUEST).into_response(),
    };

    match update_solution(&mut conn, &date, &solution) {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(_) => (StatusCode::NOT_FOUND).into_response(),
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteParams {
    date: String,
}

pub async fn delete(Query(params): Query<DeleteParams>) -> impl IntoResponse {
    let mut conn = match POOL.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let date = match PuzzleDate::new(&params.date) {
        Ok(date) => date,
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    };

    if get_puzzle(&mut conn, &date).is_some() {
        // There is an existing puzzle
        match delete_solution(&mut conn, &date) {
            Ok(_) => (StatusCode::NO_CONTENT).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    } else {
        // There isn't any existing puzzles
        (StatusCode::NOT_FOUND).into_response()
    }
}
