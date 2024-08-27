use alloy_primitives::{keccak256, BlockNumber, B256};
use revm::primitives::BlockEnv;
use serde::{Deserialize, Serialize};

use crate::block_header::casting_utils::try_downcast;
use crate::block_header::{EvmBlockHeader, Hashable};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeBlockHeader {
    number: BlockNumber,
    state_root: B256,
}

impl ForgeBlockHeader {
    pub fn new(number: BlockNumber, state_root: B256) -> Self {
        Self { number, state_root }
    }
}

impl TryFrom<&dyn EvmBlockHeader> for ForgeBlockHeader {
    type Error = &'static str;

    fn try_from(header: &dyn EvmBlockHeader) -> Result<Self, Self::Error> {
        try_downcast(header)
    }
}

impl Hashable for ForgeBlockHeader {
    fn hash_slow(&self) -> B256 {
        keccak256(self.number.to_string())
    }
}

impl EvmBlockHeader for ForgeBlockHeader {
    fn parent_hash(&self) -> &B256 {
        Default::default()
    }

    fn number(&self) -> BlockNumber {
        self.number
    }

    fn timestamp(&self) -> u64 {
        Default::default()
    }

    fn state_root(&self) -> &B256 {
        &self.state_root
    }

    fn fill_block_env(&self, _blk_env: &mut BlockEnv) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forge_block_hash_is_hash_of_block_number_as_string() {
        assert_eq!(
            ForgeBlockHeader::new(1, Default::default()).hash_slow(),
            keccak256("1")
        );
        assert_eq!(
            ForgeBlockHeader::new(12345, Default::default()).hash_slow(),
            keccak256("12345")
        );
    }
}
