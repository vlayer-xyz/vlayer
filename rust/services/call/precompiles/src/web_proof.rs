use std::convert::Into;

use alloy_primitives::Bytes;
use web_proof::verifier::verify_and_parse;

use crate::helpers::{Result, map_to_fatal};

pub(super) fn verify(input: &Bytes) -> Result<Bytes> {
    let web_proof_json = std::str::from_utf8(input).map_err(map_to_fatal)?;
    let web_proof = serde_json::from_str(web_proof_json).map_err(map_to_fatal)?;
    verify_and_parse(web_proof)
        .map(|x| x.abi_encode().into())
        .map_err(map_to_fatal)
}
