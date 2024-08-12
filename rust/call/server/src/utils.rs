use alloy_primitives::Address;
use server_utils::{alloy_hex_error_to_standard_hex_error, FieldValidationError};

pub(crate) fn parse_address_field(
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

pub(crate) fn parse_hex_field(
    field_name: &str,
    hex: String,
) -> Result<Vec<u8>, FieldValidationError> {
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
