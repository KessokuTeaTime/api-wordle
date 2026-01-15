//! Middleware for Cross-Origin Resource Sharing (CORS).
//!
//! See: https://infobytes.guru/articles/cors-errors-explained.html

/// Router layers for Cross-Origin Resource Sharing (CORS).
pub mod layers {
    use api_framework::static_lazy_lock;
    use axum::http::{HeaderValue, header, method::Method, request};
    use tower_http::cors::{AllowOrigin, CorsLayer};

    use crate::config::{Config as _, services::CorsConfig};

    static_lazy_lock! {
        /// The layer to handle Cross-Origin Resource Sharing (CORS).
        pub CORS: CorsLayer = CorsLayer::new()
        .allow_origin(AllowOrigin::async_predicate(|origin: HeaderValue, request_parts: &request::Parts| {
            let headers = request_parts.headers.clone();
            async move {
                tracing::trace!("CORS async_predicate called with origin: {:?}", origin);
                tracing::trace!("Request headers: {:?}", headers);
                let config = CorsConfig::read().unwrap_or_default();
                tracing::trace!("CORS origin check: {:?}", origin);
                config.contains(&origin)
            }
        }))
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::COOKIE]);
    }
}
