use axum::http::{header::AUTHORIZATION, HeaderName};
use axum::{Extension, Router};
use std::iter::once;
use std::sync::Arc;
use tower_http::{
    request_id::PropagateRequestIdLayer, sensitive_headers::SetSensitiveHeadersLayer,
    validate_request::ValidateRequestHeaderLayer,
};

use crate::features::routes;
use crate::infra::cors;
use crate::infra::database::DbPool;
use crate::infra::logging;

#[derive(Debug, Clone)]
pub struct AppState {
    db: DbPool,
}

pub async fn create_app(db: DbPool, allowed_origins: Vec<String>) -> Router {
    logging::init_logger();

    let state = Arc::new(AppState { db });

    routes::create_router()
        .layer(logging::make_trace_layer())
        .layer(SetSensitiveHeadersLayer::new(once(AUTHORIZATION)))
        .layer(PropagateRequestIdLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        .layer(cors::make_cors_layer(allowed_origins))
        .layer(Extension(state))
}
