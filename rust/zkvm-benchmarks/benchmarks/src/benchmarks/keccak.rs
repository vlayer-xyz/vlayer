use alloy_primitives::keccak256;

use crate::WorkloadResult;

pub(crate) fn empty() -> WorkloadResult {
    keccak256([]);

    Ok(())
}

pub(crate) fn one_block() -> WorkloadResult {
    keccak256([0; 32]);

    Ok(())
}

pub(crate) fn one_kb() -> WorkloadResult {
    keccak256([0; 1_024]);

    Ok(())
}
