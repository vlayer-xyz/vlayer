use crate::precompiles::{gas_used, map_to_fatal};
use alloy_primitives::Bytes;
use revm::precompile::{Precompile, PrecompileOutput, PrecompileResult};
use std::convert::Into;
use web_proof::verifier::verify_and_parse;

pub(super) const VERIFY_AND_PARSE_PRECOMPILE: Precompile =
    Precompile::Standard(verify_and_parse_run);

// TODO: set an accurate gas cost values reflecting the operation's computational complexity.
/// The base cost of the operation.
const VERIFY_AND_PARSE_BASE: u64 = 10;
/// The cost per word.
const VERIFY_AND_PARSE_PER_WORD: u64 = 1;

fn verify_and_parse_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used =
        gas_used(input.len(), VERIFY_AND_PARSE_BASE, VERIFY_AND_PARSE_PER_WORD, gas_limit)?;

    let web_proof_json = std::str::from_utf8(input).map_err(map_to_fatal)?;
    let web_proof = serde_json::from_str(web_proof_json).map_err(map_to_fatal)?;
    let web = verify_and_parse(web_proof).map_err(map_to_fatal)?;

    Ok(PrecompileOutput::new(gas_used, web.abi_encode().into()))
}
