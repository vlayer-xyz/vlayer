//! Type aliases for Ethereum.
use alloy_primitives::{Address, B64, B256, BlockHash, BlockNumber, Bloom, Bytes, U256, keccak256};
use alloy_rlp_derive::RlpEncodable;
use common::Hashable;
use revm::primitives::BlockEnv;
use serde::{Deserialize, Serialize};

use super::EvmBlockHeader;
use crate::casting_utils::try_downcast;

/// Ethereum post-merge block header.
#[derive(Debug, Clone, Serialize, Deserialize, RlpEncodable, Default, PartialEq)]
#[rlp(trailing)]
pub struct EthBlockHeader {
    /// Hash of the parent block's header.
    pub parent_hash: BlockHash,
    /// Unused 256-bit hash; always the hash of the empty list.
    pub ommers_hash: B256,
    /// Address that receives the priority fees of each transaction in the block.
    pub beneficiary: Address,
    /// Root hash of the state trie after all transactions in the block are executed.
    pub state_root: B256,
    /// Root hash of the trie containing all transactions in the block.
    pub transactions_root: B256,
    /// Root hash of the trie containing the receipts of each transaction in the block.
    pub receipts_root: B256,
    /// Bloom filter for log entries in the block.
    pub logs_bloom: Bloom,
    /// Always set to `0` as it's unused.
    pub difficulty: U256,
    /// The block number in the chain.
    pub number: BlockNumber,
    /// Maximum amount of gas consumed by the transactions within the block.
    pub gas_limit: u64,
    /// Total amount of gas used by all transactions in this block.
    pub gas_used: u64,
    /// Value corresponding to the seconds since Epoch at this block's inception.
    pub timestamp: u64,
    /// Arbitrary byte array containing extra data related to the block.
    pub extra_data: Bytes,
    /// Hash previously used for the PoW now containing the RANDAO value.
    pub mix_hash: B256,
    /// Unused 64-bit hash, always zero.
    pub nonce: B64,
    /// Base fee paid by all transactions in the block.
    pub base_fee_per_gas: U256,
    /// Root hash of the trie containing all withdrawals in the block.
    pub withdrawals_root: Option<B256>,
    /// Total amount of blob gas consumed by the transactions within the block.
    pub blob_gas_used: Option<u64>,
    /// Running total of blob gas consumed in excess of the target, prior to the block.
    pub excess_blob_gas: Option<u64>,
    /// Hash tree root of the parent beacon block for the given execution block.
    pub parent_beacon_block_root: Option<B256>,
    /// The Keccak 256-bit hash of the an RLP encoded list with each
    /// [EIP-7685] request in the block body.
    ///
    /// [EIP-7685]: https://eips.ethereum.org/EIPS/eip-7685
    pub requests_hash: Option<B256>,
}

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
        blk_env.basefee = self.base_fee_per_gas;
        blk_env.gas_limit = U256::from(self.gas_limit);
    }
}

impl Hashable for EthBlockHeader {
    fn hash_slow(&self) -> B256 {
        keccak256(alloy_rlp::encode(self))
    }
}

impl Default for Box<dyn EvmBlockHeader> {
    fn default() -> Self {
        Box::new(EthBlockHeader::default())
    }
}
