//! The API endpoints.

use crate::middleware::{auth::layers::kessoku_private_ci_authorization, logging::log_request};

use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, post},
};

pub mod dates;
pub mod internal;
pub mod root;

/// Routes an [`Router`] with the endpoints defined by this module.
pub fn route_from(mut app: Router) -> Router {
    app = route_gets(app);
    app = route_posts(app);
    app = route_deletes(app);
    app.layer(from_fn(log_request))
}

fn route_gets(app: Router) -> Router {
    app.route("/", get(root::get))
        .route("/dates", get(dates::get))
}

fn route_posts(app: Router) -> Router {
    app.route(
        "/internal/update",
        post(internal::update::post).route_layer(kessoku_private_ci_authorization()),
    )
    .route("/", post(root::post))
}

fn route_deletes(app: Router) -> Router {
    app.route("/", delete(root::delete))
}
