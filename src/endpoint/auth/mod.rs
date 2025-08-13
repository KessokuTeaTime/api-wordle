//! Endpoint `/auth`.

use axum::{
    http::{StatusCode, header},
    request::Request,
    response::IntoResponse,
};

use crate::middleware::auth::generate_paseto_token;

pub async fn get(_request: Request) -> impl IntoResponse {
    let token = generate_paseto_token().await;
    (StatusCode::OK, [(header::AUTHORIZATION, token)])
}
