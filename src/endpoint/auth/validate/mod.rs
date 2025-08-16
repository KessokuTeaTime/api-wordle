//! Endpoint `/auth/validate`.

use axum::{http::StatusCode, response::IntoResponse};

pub async fn get() -> impl IntoResponse {
    StatusCode::OK
}
