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

pub const PRECOMPILES: [PrecompileWithAddress; 9] = [
    PrecompileWithAddress(u64_to_address(0x100), verify_and_parse::VERIFY_AND_PARSE),
    PrecompileWithAddress(u64_to_address(0x101), verify_and_parse_email::VERIFY_AND_PARSE),
    PrecompileWithAddress(u64_to_address(0x102), json::GET_STRING),
    PrecompileWithAddress(u64_to_address(0x103), json::GET_INT),
    PrecompileWithAddress(u64_to_address(0x104), json::GET_BOOL),
    PrecompileWithAddress(u64_to_address(0x105), json::GET_ARRAY_LENGTH),
    PrecompileWithAddress(u64_to_address(0x110), regex::MATCH),
    PrecompileWithAddress(u64_to_address(0x111), regex::CAPTURE),
    PrecompileWithAddress(u64_to_address(0x120), url_pattern::TEST),
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
