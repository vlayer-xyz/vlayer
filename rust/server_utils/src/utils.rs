use alloy_primitives::hex::FromHexError as AlloyFromHexError;
use hex::FromHexError;

pub fn alloy_hex_error_to_standard_hex_error(err: AlloyFromHexError) -> FromHexError {
    match err {
        AlloyFromHexError::InvalidHexCharacter { c, index } => {
            FromHexError::InvalidHexCharacter { c, index }
        }
        AlloyFromHexError::InvalidStringLength => FromHexError::InvalidStringLength,
        AlloyFromHexError::OddLength => FromHexError::OddLength,
    }
}
