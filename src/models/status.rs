use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct APIStatus {
    pub version: String,
}

#[derive(Serialize)]
pub struct APIError {
    pub message: String,
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}
