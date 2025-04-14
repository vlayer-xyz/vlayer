use alloy_primitives::{Address, BlockHash, hex::FromHexError as AlloyFromHexError};
use hex::FromHexError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FieldValidationError {
    #[error("`{field}` {error} `{value}`")]
    InvalidHex {
        field: String,
        value: String,
        error: FromHexError,
    },
    #[error("`{field}` Invalid hex prefix `{value}`")]
    InvalidHexPrefix { field: String, value: String },
    #[error("`{field}` is too long `{length}` > `{limit}`")]
    LengthLimit {
        field: String,
        length: usize,
        limit: usize,
    },
}

pub fn parse_address_field(
    field_name: &str,
    address: String,
) -> Result<Address, FieldValidationError> {
    address
        .parse()
        .map_err(alloy_hex_error_to_standard_hex_error)
        .map_err(|error| FieldValidationError::InvalidHex {
            field: field_name.to_string(),
            value: address,
            error,
        })
}

pub fn parse_hash_field(field_name: &str, hash: String) -> Result<BlockHash, FieldValidationError> {
    hash.parse()
        .map_err(alloy_hex_error_to_standard_hex_error)
        .map_err(|error| FieldValidationError::InvalidHex {
            field: field_name.to_string(),
            value: hash,
            error,
        })
}

pub fn parse_hex_field(field_name: &str, hex: String) -> Result<Vec<u8>, FieldValidationError> {
    if !hex.starts_with("0x") {
        return Err(FieldValidationError::InvalidHexPrefix {
            field: field_name.to_string(),
            value: hex,
        });
    }
    hex::decode(&hex[2..]).map_err(|error| FieldValidationError::InvalidHex {
        field: field_name.to_string(),
        value: hex,
        error,
    })
}

const fn alloy_hex_error_to_standard_hex_error(err: AlloyFromHexError) -> FromHexError {
    match err {
        AlloyFromHexError::InvalidHexCharacter { c, index } => {
            FromHexError::InvalidHexCharacter { c, index }
        }
        AlloyFromHexError::InvalidStringLength => FromHexError::InvalidStringLength,
        AlloyFromHexError::OddLength => FromHexError::OddLength,
    }
}
