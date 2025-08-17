//! Middleware for Cross-Origin Resource Sharing (CORS).
//!
//! See: https://infobytes.guru/articles/cors-errors-explained.html

/// Router layers for Cross-Origin Resource Sharing (CORS).
pub mod layers {
    use api_framework::static_lazy_lock;
    use axum::http::{HeaderValue, header, method::Method, request};
    use tower_http::cors::{AllowOrigin, Any, CorsLayer};

    use crate::config::{CorsRuntimeConfig, RuntimeConfig};

    static_lazy_lock! {
        pub CORS: CorsLayer = CorsLayer::new()
        .allow_origin(AllowOrigin::async_predicate(|origin: HeaderValue, _request_parts: &request::Parts| async move {
            let config = CorsRuntimeConfig::load_or_default().await;
            config.contains(&origin)
        }))
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::COOKIE]);
    }
}
