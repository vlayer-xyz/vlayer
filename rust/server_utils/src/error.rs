use hex::FromHexError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FieldValidationError {
    #[error("{field} {value} `{error}`")]
    InvalidHex {
        field: String,
        value: String,
        error: FromHexError,
    },
    #[error("{field} Invalid hex prefix `{value}`")]
    InvalidHexPrefix { field: String, value: String },
}
