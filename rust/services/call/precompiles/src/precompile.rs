use derive_more::{Deref, Into};
use revm::precompile::{
    calc_linear_cost_u32, Error::OutOfGas, PrecompileErrors::Error, PrecompileWithAddress,
};

use crate::helpers::Result;

#[derive(Debug, Copy, Clone)]
pub enum Category {
    WebProof,
    EmailProof,
    Json,
    Regex,
    Url,
}

#[derive(Deref, Into)]
pub struct Precompile {
    #[deref]
    #[into]
    inner: PrecompileWithAddress,
    category: Category,
}

impl Precompile {
    pub const fn new(inner: PrecompileWithAddress, category: Category) -> Self {
        Self { inner, category }
    }

    pub const fn category(&self) -> Category {
        self.category
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
    if gas_used > gas_limit {
        Err(Error(OutOfGas))
    } else {
        Ok(gas_used)
    }
}
