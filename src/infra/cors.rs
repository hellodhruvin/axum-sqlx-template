use axum::http::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

pub fn make_cors_layer(allowed_origins: Vec<String>) -> CorsLayer {
    if allowed_origins.is_empty() {
        CorsLayer::permissive()
    } else {
        let origins: Vec<HeaderValue> = allowed_origins
            .into_iter()
            .map(|origin| {
                origin
                    .parse::<HeaderValue>()
                    .expect("Invalid allowed_origins value")
            })
            .collect();

        CorsLayer::new()
            .allow_origin(origins)
            .allow_headers(Any)
            .allow_methods(Any)
    }
}
