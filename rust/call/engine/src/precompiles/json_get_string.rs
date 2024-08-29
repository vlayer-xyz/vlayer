use crate::precompiles::{gas_used, map_to_other};
use alloy_primitives::Bytes;
use revm::precompile::{Precompile, PrecompileOutput, PrecompileResult};
use std::convert::Into;

pub(crate) const JSON_GET_STRING_PRECOMPILE: Precompile = Precompile::Standard(json_get_string_run);

// TODO: set an accurate gas cost values reflecting the operation's computational complexity.
/// The base cost of the operation.
const JSON_GET_STRING_BASE: u64 = 10;
/// The cost per word.
const JSON_GET_STRING_PER_WORD: u64 = 1;

fn json_get_string_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(
        input.len(),
        JSON_GET_STRING_BASE,
        JSON_GET_STRING_PER_WORD,
        gas_limit,
    )?;

    // TODO: parse body and path and extract field at path
    let result = "";

    Ok(PrecompileOutput::new(gas_used, result.into()))
}
