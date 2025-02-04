use std::convert::Into;

use alloy_primitives::Bytes;

use crate::helpers::{map_to_fatal, Result};

pub fn verify(input: &Bytes) -> Result<Bytes> {
    let parsed_email = email_proof::parse_and_verify(input).map_err(map_to_fatal)?;
    Ok(parsed_email.abi_encode().into())
}
