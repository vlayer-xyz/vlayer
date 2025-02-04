use std::convert::Into;

use alloy_primitives::Bytes;
use revm::precompile::PrecompileErrors;

use crate::helpers::map_to_fatal;

pub fn verify_and_parse_run(input: &Bytes) -> Result<Bytes, PrecompileErrors> {
    let parsed_email = email_proof::parse_and_verify(input).map_err(map_to_fatal)?;
    Ok(parsed_email.abi_encode().into())
}
