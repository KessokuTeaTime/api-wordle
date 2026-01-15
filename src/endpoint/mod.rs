//! The API endpoints.

use crate::middleware::{self, auth::authorize_paseto_token, session::validate_session_token};

use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

pub mod dates;
pub mod health;
pub mod play;
pub mod root;
pub mod validate;

/// Routes an [`Router`] with the endpoints defined by this module.
pub fn route_from(mut app: Router) -> Router {
    app = route_gets(app);
    app = route_posts(app);
    app.layer(TraceLayer::new_for_http())
        .layer(middleware::cors::layers::CORS.to_owned())
}

fn route_gets(app: Router) -> Router {
    app.route("/", get(root::get))
        .route("/health", get(health::get))
        .route("/dates", get(dates::get))
        .route("/validate", get(validate::get))
        .route(
            "/play/session",
            get(play::session::get).route_layer(from_fn(validate_session_token)),
        )
        .route(
            "/play/start",
            get(play::start::get).route_layer(from_fn(validate_session_token)),
        )
}

fn route_posts(app: Router) -> Router {
    app.route(
        "/",
        post(root::post).route_layer(from_fn(authorize_paseto_token)),
    )
    .route(
        "/play/submit",
        post(play::submit::post).route_layer(from_fn(validate_session_token)),
    )
}
