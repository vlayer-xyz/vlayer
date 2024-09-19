use crate::precompiles::{gas_used, map_to_other};
use alloy_primitives::Bytes;
use revm::precompile::{Precompile, PrecompileOutput, PrecompileResult};
use std::convert::Into;
pub(super) const VERIFY_EMAIL_PRECOMPILE: Precompile = Precompile::Standard(verify_and_parse_run);

/// The base cost of the operation.
const VERIFY_EMAIL_BASE: u64 = 10;
/// The cost per word.
const VERIFY_EMAIL_PER_WORD: u64 = 1;

fn verify_and_parse_run(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    let gas_used = gas_used(input.len(), VERIFY_EMAIL_BASE, VERIFY_EMAIL_PER_WORD, gas_limit)?;

    let parsed_email = email_proof::parse_mime(input).map_err(map_to_other)?;

    Ok(PrecompileOutput::new(gas_used, parsed_email.abi_encode().into()))
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_EMAIL: &[u8] = b"From: me\nTo:you\n\nThis is a test email.";

    #[test]
    fn test_gas_usage() {
        let input = Bytes::from_static(TEST_EMAIL);
        let PrecompileOutput { gas_used, .. } = verify_and_parse_run(&input, 1000).unwrap();

        assert_eq!(gas_used, 12);
    }
}
