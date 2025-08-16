//! Endpoint `/play/session`.

use crate::{
    cookies,
    middleware::session::{SessionToken, generate_session_token},
};

use axum::{Extension, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{CookieJar, cookie::Cookie};

pub async fn get(jar: CookieJar, session: Option<Extension<SessionToken>>) -> impl IntoResponse {
    match session {
        Some(Extension(SessionToken(session))) => {
            let mut cookie = Cookie::new(cookies::SESSION_TOKEN, session);
            cookie.unset_path();
            (StatusCode::OK, jar.add(cookie)).into_response()
        }
        None => {
            let token = generate_session_token().await;
            let mut cookie = Cookie::new(cookies::SESSION_TOKEN, token);
            cookie.unset_path();
            (StatusCode::CREATED, jar.add(cookie)).into_response()
        }
    }
}
