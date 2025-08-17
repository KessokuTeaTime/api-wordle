//! Endpoint `/play/session`.

use std::net::SocketAddr;

use crate::{
    cookies,
    middleware::session::{SessionToken, generate_session_token},
};

use axum::{
    Extension,
    extract::{ConnectInfo, Query},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GetParams {
    dev: Option<bool>,
}

pub async fn get(
    jar: CookieJar,
    Query(params): Query<GetParams>,
    session: Option<Extension<SessionToken>>,
) -> impl IntoResponse {
    fn setup_cookie(session: String, secure: bool) -> Cookie<'static> {
        let mut cookie = Cookie::new(cookies::SESSION_TOKEN, session);
        cookie.set_http_only(true);
        cookie.set_same_site(SameSite::None);
        cookie.set_secure(secure);
        cookie
    }

    match session {
        Some(Extension(SessionToken(session))) => (
            StatusCode::OK,
            jar.add(setup_cookie(session, !params.dev.unwrap_or(false))),
        )
            .into_response(),
        None => {
            let token = generate_session_token().await;
            (
                StatusCode::CREATED,
                jar.add(setup_cookie(token, !params.dev.unwrap_or(false))),
            )
                .into_response()
        }
    }
}
