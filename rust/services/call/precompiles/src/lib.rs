mod json;
mod regex;
mod url_pattern;
mod verify_and_parse;
pub mod verify_and_parse_email;

use revm::{
    precompile::{
        calc_linear_cost_u32, u64_to_address, Error::OutOfGas, PrecompileErrors::Error,
        PrecompileWithAddress,
    },
    primitives::PrecompileErrors,
};

use crate::{
    json::{
        GET_ARRAY_LENGTH_PRECOMPILE as JSON_GET_ARRAY_LENGTH_PRECOMPILE,
        GET_BOOL_PRECOMPILE as JSON_GET_BOOL_PRECOMPILE,
        GET_INT_PRECOMPILE as JSON_GET_INT_PRECOMPILE,
        GET_STRING_PRECOMPILE as JSON_GET_STRING_PRECOMPILE,
    },
    regex::{
        CAPTURE_PRECOMPILE as REGEX_CAPTURE_PRECOMPILE, MATCH_PRECOMPILE as REGEX_MATCH_PRECOMPILE,
    },
    url_pattern::PRECOMPILE as URL_PATTERN_TEST_PRECOMPILE,
    verify_and_parse::PRECOMPILE as VERIFY_AND_PARSE_PRECOMPILE,
    verify_and_parse_email::PRECOMPILE as VERIFY_EMAIL_PRECOMPILE,
};

pub const VLAYER_PRECOMPILES: [PrecompileWithAddress; 9] = [
    PrecompileWithAddress(u64_to_address(0x100), VERIFY_AND_PARSE_PRECOMPILE),
    PrecompileWithAddress(u64_to_address(0x101), VERIFY_EMAIL_PRECOMPILE),
    PrecompileWithAddress(u64_to_address(0x102), JSON_GET_STRING_PRECOMPILE),
    PrecompileWithAddress(u64_to_address(0x103), JSON_GET_INT_PRECOMPILE),
    PrecompileWithAddress(u64_to_address(0x104), JSON_GET_BOOL_PRECOMPILE),
    PrecompileWithAddress(u64_to_address(0x105), JSON_GET_ARRAY_LENGTH_PRECOMPILE),
    PrecompileWithAddress(u64_to_address(0x110), REGEX_MATCH_PRECOMPILE),
    PrecompileWithAddress(u64_to_address(0x111), REGEX_CAPTURE_PRECOMPILE),
    PrecompileWithAddress(u64_to_address(0x120), URL_PATTERN_TEST_PRECOMPILE),
];

#[allow(clippy::needless_pass_by_value)] // More convenient to use in map_err
fn map_to_fatal<E: ToString>(err: E) -> PrecompileErrors {
    PrecompileErrors::Fatal {
        msg: err.to_string(),
    }
}

fn gas_used(len: usize, base: u64, word: u64, gas_limit: u64) -> Result<u64, PrecompileErrors> {
    let gas_used = calc_linear_cost_u32(len, base, word);
    if gas_used > gas_limit {
        Err(Error(OutOfGas))
    } else {
        Ok(gas_used)
    }
}
