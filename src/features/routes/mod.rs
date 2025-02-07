use axum::Router;

pub mod status;
pub mod v1;

use crate::features::routes;

pub fn create_router() -> Router {
    Router::new()
        .merge(routes::status::create_route())
        .merge(Router::new().nest("/v1", routes::v1::create_router()))
}
