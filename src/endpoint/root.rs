//! Endpoint root.

use std::error::Error as _;

use api_framework::{
    framework::{State, retry_if_possible},
    unwrap,
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use diesel::PgConnection;
use serde::Deserialize;
use tracing::{error, info};

use crate::database::{
    POOL,
    puzzles::{delete_puzzle, get_puzzle, insert_or_update_puzzle},
    types::{PuzzleDate, PuzzleWord},
};

#[derive(Debug, Deserialize)]
struct RandomWord {
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
pub struct Params {
    date: PuzzleDate,
}

pub async fn get(Query(params): Query<Params>) -> impl IntoResponse {
    let mut conn = match POOL.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    if let Some(puzzle) = get_puzzle(&mut conn, &params.date) {
        (StatusCode::OK, puzzle.puzzle.to_string()).into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

pub async fn post(Query(params): Query<Params>) -> impl IntoResponse {
    let mut conn = match POOL.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    if let Some(puzzle) = get_puzzle(&mut conn, &params.date) {
        (StatusCode::OK, puzzle.puzzle.to_string()).into_response()
    } else {
        let mut retry: u8 = 0;
        loop {
            match post_transaction(&mut conn, &params.date).await {
                State::Success(str) => {
                    info!("transaction succeed!");
                    break (StatusCode::CREATED, str).into_response();
                }
                State::Retry => match retry_if_possible(&mut retry) {
                    Ok(_) => continue,
                    Err(_) => break (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
                },
                State::Stop => {
                    error!("transaction failed!");
                    break (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
            }
        }
    }
}

async fn post_transaction(conn: &mut PgConnection, date: &PuzzleDate) -> State<String> {
    let str = unwrap!(fetch_random_word().await);
    let word = match PuzzleWord::new(&str) {
        Ok(word) => word,
        Err(err) => {
            error!("{}", err);
            return State::Retry;
        }
    };

    insert_or_update_puzzle(conn, date, &word).map(|_| str)
}

pub async fn delete(Query(params): Query<Params>) -> impl IntoResponse {
    let mut conn = match POOL.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    delete_puzzle(&mut conn, &params.date);
    (StatusCode::OK).into_response()
}
