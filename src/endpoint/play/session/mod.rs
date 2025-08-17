//! Endpoint `/play/session`.

use crate::{
    cookies,
    middleware::session::{SessionToken, generate_session_token},
};

use axum::{Extension, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{CookieJar, cookie::Cookie};

pub async fn get(jar: CookieJar, session: Option<Extension<SessionToken>>) -> impl IntoResponse {
    fn setup_cookie(session: String) -> Cookie<'static> {
        let mut cookie = Cookie::new(cookies::SESSION_TOKEN, session);
        cookie.set_http_only(true);
        cookie.set_same_site(None);
        cookie.set_secure(true);
        cookie
    }

    match session {
        Some(Extension(SessionToken(session))) => {
            (StatusCode::OK, jar.add(setup_cookie(session))).into_response()
        }
        None => {
            let token = generate_session_token().await;
            (StatusCode::CREATED, jar.add(setup_cookie(token))).into_response()
        }
    }
}
