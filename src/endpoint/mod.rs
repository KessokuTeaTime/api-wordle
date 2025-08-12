//! The API endpoints.

use crate::middleware::{auth::layers::kessoku_private_ci_authorization, logging::log_request};

use axum::{Router, middleware::from_fn, routing::post};

pub mod internal;
pub mod wordle;

/// Routes an [`Router`] with the endpoints defined by this module.
pub fn route_from(mut app: Router) -> Router {
    app = route_gets(app);
    app = route_posts(app);
    app.layer(from_fn(log_request))
}

fn route_gets(app: Router) -> Router {
    app
}

fn route_posts(app: Router) -> Router {
    app.route(
        "/api/internal/update/wordle",
        post(internal::update::wordle::post).route_layer(kessoku_private_ci_authorization()),
    )
}
