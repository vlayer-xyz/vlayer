use crate::error::AppError;
use alloy_primitives::Address;
use server_utils::{alloy_hex_error_to_standard_hex_error, FieldValidationError};

pub(crate) fn parse_address_field(field_name: &str, address: String) -> Result<Address, AppError> {
    address
        .parse()
        .map_err(alloy_hex_error_to_standard_hex_error)
        .map_err(|err| {
            AppError::FieldValidation(
                field_name.to_string(),
                FieldValidationError::InvalidHex(address, err),
            )
        })
}

pub(crate) fn parse_hex_field(field_name: &str, hex: String) -> Result<Vec<u8>, AppError> {
    if !hex.starts_with("0x") {
        return Err(AppError::FieldValidation(
            field_name.to_string(),
            FieldValidationError::InvalidHexPrefix(hex),
        ));
    }
    hex::decode(&hex[2..]).map_err(|err| {
        AppError::FieldValidation(
            field_name.to_string(),
            FieldValidationError::InvalidHex(hex, err),
        )
    })
}
