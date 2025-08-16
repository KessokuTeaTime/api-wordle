//! The API endpoints.

use crate::middleware::{
    auth::{authorize_paseto_token, layers},
    session::validate_session_token,
};

use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, post, put},
};
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod dates;
pub mod internal;
pub mod play;
pub mod root;
pub mod validate;

/// Routes an [`Router`] with the endpoints defined by this module.
pub fn route_from(mut app: Router) -> Router {
    app = route_gets(app);
    app = route_posts(app);
    app = route_puts(app);
    app = route_deletes(app);
    app.layer(TraceLayer::new_for_http())
}

fn route_gets(app: Router) -> Router {
    app.route("/", get(root::get))
        .route("/dates", get(dates::get))
        .route("/validate", get(validate::get))
        .route(
            "/auth",
            get(auth::get).route_layer(layers::admin_password_authorization()),
        )
        .route(
            "/auth/validate",
            get(auth::validate::get).route_layer(from_fn(authorize_paseto_token)),
        )
        .route(
            "/play/session",
            get(play::session::get).route_layer(from_fn(validate_session_token)),
        )
}

fn route_posts(app: Router) -> Router {
    app.route(
        "/",
        post(root::post).route_layer(from_fn(authorize_paseto_token)),
    )
    .route(
        "/internal/update",
        post(internal::update::post).route_layer(layers::kessoku_private_ci_authorization()),
    )
}

fn route_puts(app: Router) -> Router {
    app.route(
        "/",
        put(root::put).route_layer(from_fn(authorize_paseto_token)),
    )
}

fn route_deletes(app: Router) -> Router {
    app.route(
        "/",
        delete(root::delete).route_layer(from_fn(authorize_paseto_token)),
    )
}
