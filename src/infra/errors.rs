use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("...")]
pub enum AppError {
    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    SqlxError(#[from] sqlx::Error),
}

impl AppError {
    fn get_codes(&self) -> (StatusCode, String) {
        match *self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found".to_string()),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_reques".to_string()),
            AppError::SqlxError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "sqlx_error".to_string()),
        }
    }

    pub fn not_found() -> Self {
        AppError::NotFound(NotFound {})
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, code) = self.get_codes();
        let message = self.to_string();
        let body = Json(json!({ "code": code.to_uppercase(), "message": message }));

        (status_code, body).into_response()
    }
}

#[derive(Error, Debug)]
#[error("Bad request")]
pub struct BadRequest {}

#[derive(Error, Debug)]
#[error("Not found")]
pub struct NotFound {}
