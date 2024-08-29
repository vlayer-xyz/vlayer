use crate::precompiles::{gas_used, map_to_other};
use alloy_primitives::Bytes;
use revm::precompile::{Precompile, PrecompileOutput, PrecompileResult};
use std::convert::Into;

pub(crate) const GET_STRING_PRECOMPILE: Precompile = Precompile::Standard(get_string_run);

// TODO: set an accurate gas cost values reflecting the operation's computational complexity.
/// The base cost of the operation.
const GET_STRING_BASE: u64 = 10;
/// The cost per word.
const GET_STRING_PER_WORD: u64 = 1;

fn get_string_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), GET_STRING_BASE, GET_STRING_PER_WORD, gas_limit)?;

    // TODO: parse body and path
    let _body_json = std::str::from_utf8(input).map_err(map_to_other)?;

    // TODO extract string field from body at path
    let result = "";

    Ok(PrecompileOutput::new(gas_used, result.into()))
}
