//! The API endpoints.

use crate::middleware::auth::{
    authorize_paseto_token,
    layers::{admin_password_authorization, kessoku_private_ci_authorization},
};

use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod dates;
pub mod internal;
pub mod root;

/// Routes an [`Router`] with the endpoints defined by this module.
pub fn route_from(app: Router) -> Router {
    app.route(
        "/",
        get(root::get)
            .post(root::post)
            .put(root::put)
            .delete(root::delete),
    )
    .route("/dates", get(dates::get))
    .route(
        "/auth",
        get(auth::get).route_layer(admin_password_authorization()),
    )
    .route(
        "/auth/test",
        get(auth::get).route_layer(from_fn(authorize_paseto_token)),
    )
    .route(
        "/internal/update",
        post(internal::update::post).route_layer(kessoku_private_ci_authorization()),
    )
    .layer(TraceLayer::new_for_http())
}
