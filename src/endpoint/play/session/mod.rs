//! Endpoint `/play/session`.

use crate::middleware::session::generate_session_token;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn get() -> impl IntoResponse {
    let token = generate_session_token().await;
    (
        StatusCode::OK,
        Json(json!({
            "token": token
        })),
    )
}
