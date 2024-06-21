use alloy_primitives::hex::FromHexError;
use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::json::Json;

// Format in which errors are returned to the user
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ErrorResponse {
    message: String,
}

#[cfg(test)]
impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq)]
pub enum AppError {
    #[error("The request body contained invalid JSON")]
    JsonRejection(
        #[from]
        #[derivative(PartialEq = "ignore")]
        JsonRejection,
    ),
    #[error("Invalid address: {field} -> {error}")]
    InvalidAddress { field: String, error: FromHexError },
}

// Tell axum how `AppError` should be converted into a response
//
// This is also a convenient place to log errors
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            // Our fault
            // User fault - these errors are caused by bad user input so don't log it
            AppError::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            AppError::InvalidAddress { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}
