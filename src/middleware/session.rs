//! Middleware for session creating and validating.

use crate::{cookies, env::SESSION_SYMMETRIC_KEY};

use std::net::SocketAddr;

use axum::{
    Extension,
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use rusty_paseto::{
    core::{Key, Local, PasetoSymmetricKey, V4},
    prelude::{PasetoBuilder, PasetoParser},
};
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionToken(String);

pub async fn generate_session_token() -> String {
    info!("generating session token…");
    let key: PasetoSymmetricKey<_, _> = Key::from(*SESSION_SYMMETRIC_KEY).into();

    PasetoBuilder::<V4, Local>::default()
        .set_no_expiration_danger_acknowledged()
        .build(&key)
        .unwrap()
}

pub async fn validate_session_token(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Response {
    info!("validating session token for {addr}…");

    let token = match jar.get(cookies::SESSION_TOKEN) {
        Some(cookie) => cookie.value(),
        None => {
            info!("failed to validate {addr}: token not found");
            return (StatusCode::NOT_FOUND).into_response();
        }
    };

    let key: PasetoSymmetricKey<_, _> = Key::from(*SESSION_SYMMETRIC_KEY).into();
    let _json_value = match PasetoParser::<V4, Local>::new().parse(token, &key) {
        Ok(json_value) => {
            info!("validated {addr} with session token {token}!");
            json_value
        }
        Err(_) => {
            info!("failed to validate {addr}: cannot parse token");
            return (StatusCode::NOT_FOUND).into_response();
        }
    };

    (
        Extension(SessionToken(token.to_owned())),
        next.run(request).await,
    )
        .into_response()
}
