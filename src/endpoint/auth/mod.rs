//! Endpoint `/auth`.

use axum::{
    http::{StatusCode, header},
    response::IntoResponse,
};

use crate::middleware::auth::generate_paseto_token;

pub async fn get() -> impl IntoResponse {
    let token = generate_paseto_token().await;
    (StatusCode::OK, [(header::AUTHORIZATION, token)])
}
