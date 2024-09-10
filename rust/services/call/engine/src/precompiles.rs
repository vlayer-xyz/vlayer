mod json_get_string;
mod verify_and_parse;
mod verify_and_parse_email;

use crate::precompiles::json_get_string::JSON_GET_STRING_PRECOMPILE;
use crate::precompiles::verify_and_parse::VERIFY_AND_PARSE_PRECOMPILE;
use crate::precompiles::verify_and_parse_email::VERIFY_EMAIL_PRECOMPILE;
use revm::precompile::Error::OutOfGas;
use revm::precompile::{calc_linear_cost_u32, u64_to_address};
use revm::{
    precompile::{PrecompileError::Other, PrecompileErrors::Error, PrecompileWithAddress},
    primitives::PrecompileErrors,
};

pub(crate) const VLAYER_PRECOMPILES: [PrecompileWithAddress; 3] = [
    PrecompileWithAddress(u64_to_address(0x100), VERIFY_AND_PARSE_PRECOMPILE),
    PrecompileWithAddress(u64_to_address(0x101), JSON_GET_STRING_PRECOMPILE),
    PrecompileWithAddress(u64_to_address(0x102), VERIFY_EMAIL_PRECOMPILE),
];

fn map_to_other<E: ToString>(err: E) -> PrecompileErrors {
    Error(Other(err.to_string()))
}

fn gas_used(len: usize, base: u64, word: u64, gas_limit: u64) -> Result<u64, PrecompileErrors> {
    let gas_used = calc_linear_cost_u32(len, base, word);
    if gas_used > gas_limit {
        Err(Error(OutOfGas))
    } else {
        Ok(gas_used)
    }
}
