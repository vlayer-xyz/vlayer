use alloy_primitives::{keccak256, B256};

pub fn hash(data: impl AsRef<[u8]>) -> B256 {
    #[allow(clippy::disallowed_methods)]
    keccak256(data)
}
