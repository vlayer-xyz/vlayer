use sha2::{Digest, Sha256};

use crate::WorkloadResult;

pub(crate) fn empty() -> WorkloadResult {
    Sha256::digest([]);

    Ok(())
}

pub(crate) fn one_block() -> WorkloadResult {
    Sha256::digest([0; 32]);

    Ok(())
}

pub(crate) fn one_kb() -> WorkloadResult {
    Sha256::digest([0; 1_024]);

    Ok(())
}

pub(crate) fn eight_kb() -> WorkloadResult {
    Sha256::digest([0; 8_192]);

    Ok(())
}
