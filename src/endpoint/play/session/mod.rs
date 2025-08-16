//! Endpoint `/play/session`.

use crate::middleware::session::{SessionToken, generate_session_token};

use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone, Serialize)]
pub struct GetResponse {
    token: String,
}

pub async fn get(token_extension: Option<Extension<SessionToken>>) -> impl IntoResponse {
    match token_extension {
        Some(Extension(SessionToken(token))) => {
            (StatusCode::OK, Json(GetResponse { token })).into_response()
        }
        None => {
            let token = generate_session_token().await;
            (StatusCode::CREATED, Json(GetResponse { token })).into_response()
        }
    }
}
