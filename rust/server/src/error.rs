use axum::{
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use crate::json::AppJson;

// Format in which errors are returned to the user
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("The request body contained invalid JSON")]
    JsonRejection(#[from] JsonRejection),
}

// Tell axum how `AppError` should be converted into a response
//
// This is also a convenient place to log errors
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::JsonRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                (rejection.status(), rejection.body_text())
            }
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}
