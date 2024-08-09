use hex::FromHexError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FieldValidationError {
    #[error("{1} `{0}`")]
    InvalidHex(String, FromHexError),
    #[error("Invalid hex prefix `{0}`")]
    InvalidHexPrefix(String),
}
