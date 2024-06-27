//! Type aliases for Ethereum.
use crate::{EvmEnv, SolCommitment};

use super::{EvmBlockHeader, EvmInput};
use alloy_primitives::{
    keccak256, Address, BlockHash, BlockNumber, Bloom, Bytes, Sealable, B256, B64, U256,
};
use alloy_rlp_derive::RlpEncodable;
use ethers_core::types::Block;
use revm::primitives::BlockEnv;
use serde::{Deserialize, Serialize};

/// [EvmEnv] for Ethereum.
pub type EthEvmEnv<D> = EvmEnv<D, EthBlockHeader>;

/// [EvmInput] for Ethereum.
pub type EthEvmInput = EvmInput<EthBlockHeader>;

/// Ethereum post-merge block header.
#[derive(Debug, Clone, Serialize, Deserialize, RlpEncodable)]
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
}

impl Sealable for EthBlockHeader {
    #[inline]
    fn hash_slow(&self) -> B256 {
        keccak256(alloy_rlp::encode(self))
    }
}

impl EvmBlockHeader for EthBlockHeader {
    #[inline]
    fn parent_hash(&self) -> &B256 {
        &self.parent_hash
    }
    #[inline]
    fn number(&self) -> BlockNumber {
        self.number
    }
    #[inline]
    fn timestamp(&self) -> u64 {
        self.timestamp
    }
    #[inline]
    fn state_root(&self) -> &B256 {
        &self.state_root
    }

    #[inline]
    /// Returns the [SolCommitment] used to validate the environment.
    fn block_commitment(
        &self,
        start_contract_address: Address,
        function_selector: [u8; 4],
    ) -> SolCommitment {
        SolCommitment {
            startContractAddress: start_contract_address,
            functionSelector: function_selector.into(),
            settleBlockHash: self.clone().seal_slow().seal(),
            settleBlockNumber: U256::from(self.number()),
        }
    }

    #[inline]
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

impl<T> TryFrom<Block<T>> for EthBlockHeader {
    type Error = String;

    fn try_from(block: Block<T>) -> Result<Self, Self::Error> {
        Ok(EthBlockHeader {
            parent_hash: from_ethers_h256(block.parent_hash),
            ommers_hash: from_ethers_h256(block.uncles_hash),
            beneficiary: block.author.ok_or("author missing")?.0.into(),
            state_root: from_ethers_h256(block.state_root),
            transactions_root: from_ethers_h256(block.transactions_root),
            receipts_root: from_ethers_h256(block.receipts_root),
            logs_bloom: alloy_primitives::Bloom::from_slice(
                block.logs_bloom.ok_or("bloom missing")?.as_bytes(),
            ),
            difficulty: from_ethers_u256(block.difficulty),
            number: block.number.ok_or("number is missing")?.as_u64(),
            gas_limit: block
                .gas_limit
                .try_into()
                .map_err(|_| "invalid gas limit")?,
            gas_used: block.gas_used.try_into().map_err(|_| "invalid gas used")?,
            timestamp: block
                .timestamp
                .try_into()
                .map_err(|_| "invalid timestamp")?,
            extra_data: block.extra_data.0.into(),
            mix_hash: from_ethers_h256(block.mix_hash.ok_or("mix_hash is missing")?),
            nonce: block.nonce.ok_or("nonce is missing")?.0.into(),
            base_fee_per_gas: from_ethers_u256(
                block
                    .base_fee_per_gas
                    .ok_or("base_fee_per_gas is missing")?,
            ),
            withdrawals_root: block.withdrawals_root.map(from_ethers_h256),
            blob_gas_used: block.blob_gas_used.map(|x| x.try_into()).transpose()?,
            excess_blob_gas: block.excess_blob_gas.map(|x| x.try_into()).transpose()?,
            parent_beacon_block_root: block.parent_beacon_block_root.map(from_ethers_h256),
        })
    }
}

pub fn from_ethers_bytes(v: ethers_core::types::Bytes) -> alloy_primitives::Bytes {
    v.0.into()
}

pub fn to_ethers_h256(v: alloy_primitives::B256) -> ethers_core::types::H256 {
    v.0.into()
}

pub fn from_ethers_h256(v: ethers_core::types::H256) -> B256 {
    v.0.into()
}

pub fn from_ethers_u256(v: ethers_core::types::U256) -> U256 {
    alloy_primitives::U256::from_limbs(v.0)
}

pub fn to_ethers_h160(v: alloy_primitives::Address) -> ethers_core::types::H160 {
    v.into_array().into()
}
