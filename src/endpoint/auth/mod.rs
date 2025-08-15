//! Endpoint `/auth`.

use crate::middleware::auth::generate_paseto_token;

use axum::{
    http::{StatusCode, header},
    response::IntoResponse,
};

pub mod validate;

pub async fn get() -> impl IntoResponse {
    let token = generate_paseto_token().await;
    (StatusCode::OK, [(header::AUTHORIZATION, token)])
}
