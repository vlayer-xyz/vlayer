use alloy_primitives::{BlockNumber, B256};
use alloy_rlp::Encodable;
use revm::primitives::BlockEnv;
use serde::{Deserialize, Serialize};

use crate::{casting_utils::try_downcast, EvmBlockHeader};

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

impl Encodable for ForgeBlockHeader {
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        out.put_slice(self.number.to_string().as_bytes());
    }
}

impl TryFrom<&dyn EvmBlockHeader> for ForgeBlockHeader {
    type Error = &'static str;

    fn try_from(header: &dyn EvmBlockHeader) -> Result<Self, Self::Error> {
        try_downcast(header)
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
    use alloy_primitives::keccak256;

    use super::*;
    use crate::Hashable;

    #[test]
    fn test_forge_block_hash_is_hash_of_block_number_as_string() {
        assert_eq!(ForgeBlockHeader::new(1, Default::default()).hash_slow(), keccak256("1"));
        assert_eq!(
            ForgeBlockHeader::new(12345, Default::default()).hash_slow(),
            keccak256("12345")
        );
    }
}
