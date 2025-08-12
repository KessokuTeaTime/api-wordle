//! The API endpoints.

use crate::middleware::{auth::layers::kessoku_private_ci_authorization, logging::log_request};

use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};

pub mod dates;
pub mod generate;
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
    .route("/generate", post(generate::post))
    .route(
        "/internal/update",
        post(internal::update::post).route_layer(kessoku_private_ci_authorization()),
    )
    .layer(from_fn(log_request))
}
