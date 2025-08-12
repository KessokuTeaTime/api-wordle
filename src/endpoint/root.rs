//! Endpoint root.

use std::error::Error as _;

use api_framework::framework::State;
use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use tracing::{error, info};

use crate::database::{
    POOL,
    puzzles::{delete_solution, get_puzzle, get_puzzles, insert_solution, update_solution},
    types::{NewPuzzle, Puzzle, PuzzleDate, PuzzleSolution},
};

#[derive(Debug, Deserialize)]
pub struct RandomWord {
    word: String,
    length: usize,
    category: String,
    language: String,
}

pub async fn fetch_random_word() -> State<String> {
    let url = "https://random-words-api.kushcreates.com/api?language=en&category=wordle&length=5&type=lowercase&words=1";
    let request_builder = reqwest::Client::new().get(url).query(&[
        ("language", "en"),
        ("category", "wordle"),
        ("length", "5"),
        ("type", "lowercase"),
        ("words", "1"),
    ]);

    let response = match request_builder.send().await {
        Ok(response) => response,
        Err(err) => {
            error!("failed to fetch artifacts from {url}: {err}");
            return match err {
                _ if err.is_connect() || err.is_timeout() => State::Retry,
                _ => State::Stop,
            };
        }
    };

    match response.json::<Vec<RandomWord>>().await {
        Ok(random_words) => match &random_words[..] {
            [] => {
                error!("invalid random word data: no word is found!");
                State::Stop
            }
            [
                RandomWord {
                    word,
                    length: _,
                    category: _,
                    language: _,
                },
                ..,
            ] => {
                info!("fetched random word {}", word);
                State::Success(word.to_owned())
            }
        },
        Err(err) => {
            error!("failed to parse data from {url}: {err}");

            if let Some(source) = err.source() {
                error!("{source}")
            }

            State::Retry
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetParams {
    date: Option<String>,
}

pub async fn get(Query(params): Query<GetParams>) -> impl IntoResponse {
    let mut conn = match POOL.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    if let Some(date) = params.date {
        match PuzzleDate::new(&date) {
            Ok(date) => {
                if let Some(puzzle) = get_puzzle(&mut conn, &date) {
                    (StatusCode::OK, Json(puzzle.to_new_puzzle())).into_response()
                } else {
                    (StatusCode::NOT_FOUND).into_response()
                }
            }
            Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        }
    } else {
        let puzzles: Vec<NewPuzzle> = get_puzzles(&mut conn, false)
            .into_iter()
            .map(Puzzle::to_new_puzzle)
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

    match (
        PuzzleDate::new(&payload.date),
        PuzzleSolution::new(&payload.solution),
    ) {
        (Ok(date), Ok(solution)) => match insert_solution(&mut conn, &date, &solution) {
            Ok(_) => (StatusCode::CREATED).into_response(),
            Err(_) => (StatusCode::CONFLICT).into_response(),
        },
        _ => (StatusCode::BAD_REQUEST).into_response(),
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

    match (
        PuzzleDate::new(&payload.date),
        PuzzleSolution::new(&payload.solution),
    ) {
        (Ok(date), Ok(solution)) => match update_solution(&mut conn, &date, &solution) {
            Ok(_) => (StatusCode::CREATED).into_response(),
            Err(_) => (StatusCode::NOT_FOUND).into_response(),
        },
        _ => (StatusCode::BAD_REQUEST).into_response(),
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

    match PuzzleDate::new(&params.date) {
        Ok(date) => {
            if get_puzzle(&mut conn, &date).is_some() {
                // There is an existing puzzle
                match delete_solution(&mut conn, &date) {
                    Ok(_) => (StatusCode::NO_CONTENT).into_response(),
                    Err(err) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                    }
                }
            } else {
                // There isn't any existing puzzles
                (StatusCode::NOT_FOUND).into_response()
            }
        }
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
