use crate::error::{AppError, FieldValidationError};
use alloy_primitives::hex::FromHexError as AlloyFromHexError;
use alloy_primitives::Address;
use hex::FromHexError;

pub(crate) fn parse_address_field(field_name: &str, address: String) -> Result<Address, AppError> {
    address
        .parse()
        .map_err(alloy_hex_error_to_standard_hex_error)
        .map_err(|err| {
            AppError::FieldValidationError(
                field_name.to_string(),
                FieldValidationError::InvalidHex(address, err),
            )
        })
}

pub(crate) fn parse_hex_field(field_name: &str, hex: String) -> Result<Vec<u8>, AppError> {
    if !hex.starts_with("0x") {
        return Err(AppError::FieldValidationError(
            field_name.to_string(),
            FieldValidationError::InvalidHexPrefix(hex),
        ));
    }
    hex::decode(&hex[2..]).map_err(|err| {
        AppError::FieldValidationError(
            field_name.to_string(),
            FieldValidationError::InvalidHex(hex, err),
        )
    })
}

fn alloy_hex_error_to_standard_hex_error(err: AlloyFromHexError) -> FromHexError {
    match err {
        AlloyFromHexError::InvalidHexCharacter { c, index } => {
            FromHexError::InvalidHexCharacter { c, index }
        }
        AlloyFromHexError::InvalidStringLength => FromHexError::InvalidStringLength,
        AlloyFromHexError::OddLength => FromHexError::OddLength,
    }
}
