use std::convert::Into;

use alloy_primitives::Bytes;
use revm::precompile::{Precompile, PrecompileOutput, PrecompileResult};

use crate::precompiles::{gas_used, map_to_fatal};

pub(super) const VERIFY_EMAIL_PRECOMPILE: Precompile = Precompile::Standard(verify_and_parse_run);

/// The base cost of the operation.
const VERIFY_EMAIL_BASE: u64 = 10;
/// The cost per word.
const VERIFY_EMAIL_PER_WORD: u64 = 1;

pub fn verify_and_parse_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), VERIFY_EMAIL_BASE, VERIFY_EMAIL_PER_WORD, gas_limit)?;

    let parsed_email = email_proof::parse_and_verify(input).map_err(map_to_fatal)?;

    Ok(PrecompileOutput::new(gas_used, parsed_email.abi_encode().into()))
}
