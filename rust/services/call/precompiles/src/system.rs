use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;

use crate::helpers::Result;

pub(super) fn is_vlayer_test(_: &Bytes) -> Result<Bytes> {
    Ok(true.abi_encode().into())
}
