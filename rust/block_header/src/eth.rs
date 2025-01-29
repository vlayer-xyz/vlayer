//! Type aliases for Ethereum.
pub use alloy_consensus::Header as EthBlockHeader;
use alloy_primitives::{BlockNumber, B256, U256};
use revm::primitives::BlockEnv;

use super::EvmBlockHeader;
use crate::casting_utils::try_downcast;

impl TryFrom<&dyn EvmBlockHeader> for EthBlockHeader {
    type Error = &'static str;

    fn try_from(header: &dyn EvmBlockHeader) -> Result<Self, Self::Error> {
        try_downcast(header)
    }
}

impl EvmBlockHeader for EthBlockHeader {
    fn parent_hash(&self) -> &B256 {
        &self.parent_hash
    }

    fn number(&self) -> BlockNumber {
        self.number
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn state_root(&self) -> &B256 {
        &self.state_root
    }

    fn fill_block_env(&self, blk_env: &mut BlockEnv) {
        blk_env.number = U256::from(self.number);
        blk_env.coinbase = self.beneficiary;
        blk_env.timestamp = U256::from(self.timestamp);
        blk_env.difficulty = self.difficulty;
        blk_env.prevrandao = Some(self.mix_hash);
        blk_env.basefee = U256::from(self.base_fee_per_gas.unwrap_or_default());
        blk_env.gas_limit = U256::from(self.gas_limit);
    }
}

impl Default for Box<dyn EvmBlockHeader> {
    fn default() -> Self {
        Box::new(EthBlockHeader::default())
    }
}
