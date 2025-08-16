//! Endpoint `/play/session`.

use crate::middleware::session::{SessionToken, generate_session_token};

use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GetResponse {
    token: String,
}

pub async fn get(session: Option<Extension<SessionToken>>) -> impl IntoResponse {
    match session {
        Some(Extension(SessionToken(session))) => {
            (StatusCode::OK, Json(GetResponse { token: session })).into_response()
        }
        None => {
            let token = generate_session_token().await;
            (StatusCode::CREATED, Json(GetResponse { token })).into_response()
        }
    }
}
