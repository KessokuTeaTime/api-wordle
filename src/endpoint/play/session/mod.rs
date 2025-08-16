//! Endpoint `/play/session`.

use crate::{
    cookies,
    middleware::session::{SessionToken, generate_session_token},
};

use axum::{Extension, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{CookieJar, cookie::Cookie};

pub async fn get(jar: CookieJar, session: Option<Extension<SessionToken>>) -> impl IntoResponse {
    match session {
        Some(Extension(SessionToken(session))) => (
            StatusCode::OK,
            jar.add(Cookie::new(cookies::SESSION_TOKEN, session)),
        )
            .into_response(),
        None => {
            let token = generate_session_token().await;
            (
                StatusCode::CREATED,
                jar.add(Cookie::new(cookies::SESSION_TOKEN, token)),
            )
                .into_response()
        }
    }
}
