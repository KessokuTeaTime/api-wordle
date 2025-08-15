//! Middleware for authorization.

use crate::env::PASETO_SYMMETRIC_KEY;

use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use reqwest::StatusCode;
use rusty_paseto::{
    core::{Key, Local, PasetoSymmetricKey, V4},
    prelude::{ExpirationClaim, PasetoBuilder, PasetoParser},
};
use tracing::info;

/// Router layers for authorization.
pub mod layers {
    use crate::env::{ADMIN_PASSWORD, KTT_API_PASSWORD, KTT_API_USERNAME};
    use tower_http::auth::AddAuthorizationLayer;

    /// The layer that authorizes requests with the KessokuTeaTime private CI key in Base 64 format.
    ///
    /// See: [`KTT_API_USERNAME`], [`KTT_API_PASSWORD`], [`AddAuthorizationLayer`]
    pub fn kessoku_private_ci_authorization() -> AddAuthorizationLayer {
        AddAuthorizationLayer::basic(&KTT_API_USERNAME, &KTT_API_PASSWORD)
    }

    pub fn admin_password_authorization() -> AddAuthorizationLayer {
        AddAuthorizationLayer::bearer(&hex::encode(*ADMIN_PASSWORD))
    }
}

pub async fn generate_paseto_token() -> String {
    info!("generating PASETO token…");
    let timeout = (chrono::Local::now() + chrono::Duration::minutes(5)).to_rfc3339();
    let key: PasetoSymmetricKey<_, _> = Key::from(*PASETO_SYMMETRIC_KEY).into();

    PasetoBuilder::<V4, Local>::default()
        .set_claim(ExpirationClaim::try_from(timeout).unwrap())
        .build(&key)
        .unwrap()
}

pub async fn authorize_paseto_token(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    request: Request,
    next: Next,
) -> Response {
    info!("authorizing PASETO token for {addr}…");

    let token = bearer.token().to_owned();
    let key: PasetoSymmetricKey<_, _> = Key::from(*PASETO_SYMMETRIC_KEY).into();
    let _json_value = match PasetoParser::<V4, Local>::new().parse(&token, &key) {
        Ok(json_value) => {
            info!("authorized {addr}!");
            json_value
        }
        Err(_) => {
            info!("failed to authorize {addr}");
            return (StatusCode::UNAUTHORIZED, "token unmatch").into_response();
        }
    };

    next.run(request).await
}
