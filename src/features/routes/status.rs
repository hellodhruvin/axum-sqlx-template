use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub fn create_route() -> Router {
    Router::new().route("/status", get(get_status))
}

async fn get_status() -> Result<Json<Status>, AppError> {
    Ok(Json(Status {
        status: "ok".to_owned(),
    }))
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    status: String,
}
