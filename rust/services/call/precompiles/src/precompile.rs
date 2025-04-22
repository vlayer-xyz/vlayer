use derive_more::{Deref, Into, derive::Display};
use revm::precompile::{
    Error::OutOfGas, PrecompileErrors::Error, PrecompileWithAddress, calc_linear_cost_u32,
};
use serde::Serialize;

use crate::helpers::Result;

#[derive(Debug, Copy, Clone, Display, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Tag {
    WebProof,
    EmailProof,
    JsonGetString,
    JsonGetInt,
    JsonGetBool,
    JsonGetFloatAsInt,
    JsonGetArrayLength,
    RegexIsMatch,
    RegexCapture,
    UrlPatternTest,
    IsVlayerTest,
}

#[derive(Deref, Into, Debug, Clone)]
pub struct Precompile {
    #[deref]
    #[into]
    inner: PrecompileWithAddress,
    tag: Tag,
}

impl Precompile {
    pub const fn new(inner: PrecompileWithAddress, tag: Tag) -> Self {
        Self { inner, tag }
    }

    pub const fn tag(&self) -> Tag {
        self.tag
    }
}

pub(super) fn gas_used(
    bytes: usize,
    gas_limit: u64,
    base_cost: u64,
    byte_cost: u64,
) -> Result<u64> {
    const EVM_WORD_SIZE_BYTES: u64 = 32;
    let word_cost = byte_cost * EVM_WORD_SIZE_BYTES;
    let gas_used = calc_linear_cost_u32(bytes, base_cost, word_cost);
    (gas_used <= gas_limit)
        .then_some(gas_used)
        .ok_or(Error(OutOfGas))
}
