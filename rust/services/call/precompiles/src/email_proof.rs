use std::convert::Into;

use alloy_primitives::Bytes;

use crate::helpers::{map_to_fatal, Result};

pub fn verify(input: &Bytes) -> Result<Bytes> {
    email_proof::parse_and_verify(input)
        .map(|x| x.abi_encode().into())
        .map_err(map_to_fatal)
}
