//! Middlewares for authorization.

use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{TypedHeader, headers::authorization::Bearer};
use reqwest::StatusCode;
use rusty_paseto::{
    core::{Key, Local, PasetoSymmetricKey, V4},
    prelude::{ExpirationClaim, PasetoBuilder, PasetoParser},
};

use crate::env::PASETO_SYMMETRIC_KEY;

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
        AddAuthorizationLayer::bearer(&str::from_utf8(&ADMIN_PASSWORD[..]).unwrap())
    }
}

pub async fn generate_paseto_token() -> String {
    let in_2_minutes = (chrono::Local::now() + chrono::Duration::minutes(2)).to_rfc3339();
    let key: PasetoSymmetricKey<_, _> = Key::from(*PASETO_SYMMETRIC_KEY).into();

    PasetoBuilder::<V4, Local>::default()
        .set_claim(ExpirationClaim::try_from(in_2_minutes).unwrap())
        .build(&key)
        .unwrap()
}

/// Authorizes the request with PASETO.
///
/// 1. Client sends the hashed password to request a token, which is encrypted with PASERK using the hashed password as the key.
/// 2. Client gets the encrypted token and decrypts it.
/// 3. Client sends the decrypted token in its header to authorize.
pub async fn authorize_paseto_token(
    bearer: Option<TypedHeader<Bearer>>,
    request: Request,
    next: Next,
) -> Response {
    let token = match bearer {
        Some(bearer) => bearer.token().to_owned(),
        None => return (StatusCode::UNAUTHORIZED).into_response(),
    };
    let key: PasetoSymmetricKey<_, _> = Key::from(*PASETO_SYMMETRIC_KEY).into();
    match PasetoParser::<V4, Local>::new().parse(&token, &key) {
        Ok(_) => next.run(request).await,
        _ => (StatusCode::UNAUTHORIZED).into_response(),
    }
}
