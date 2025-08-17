//! Middleware for session creating and validating.

use crate::{cookies, env::SESSION_SYMMETRIC_KEY};

use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use rusty_paseto::{
    core::{Key, Local, PasetoSymmetricKey, V4},
    prelude::{PasetoBuilder, PasetoParser},
};
use tracing::info;

/// The session token to inject as an extension.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionToken(pub String);

/// Generates a local, symmetric session token.
///
/// # Panics
///
/// Panics if unable to generate a session token.
///
/// See: [`SESSION_SYMMETRIC_KEY`]
pub async fn generate_session_token() -> String {
    info!("generating session token…");
    let key: PasetoSymmetricKey<_, _> = Key::from(*SESSION_SYMMETRIC_KEY).into();

    PasetoBuilder::<V4, Local>::default()
        .set_no_expiration_danger_acknowledged()
        .build(&key)
        .unwrap()
}

/// Validates the session token.
pub async fn validate_session_token(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    info!("validating session token for {addr}…");

    match jar.get(cookies::SESSION_TOKEN) {
        Some(cookie) => {
            let token = cookie.value();

            let key: PasetoSymmetricKey<_, _> = Key::from(*SESSION_SYMMETRIC_KEY).into();
            match PasetoParser::<V4, Local>::new().parse(token, &key) {
                Ok(_) => {
                    info!("validated {addr} with session token {token}!");
                    request
                        .extensions_mut()
                        .insert(SessionToken(token.to_owned()));
                }
                Err(_) => {
                    info!("failed to validate {addr}: cannot parse token");
                }
            }
        }
        None => {
            info!("failed to validate {addr}: token not found");
        }
    };
    next.run(request).await
}
