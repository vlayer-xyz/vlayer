pub mod eth;

use std::any::Any;

use as_any::{AsAny, Downcast};

use alloy_primitives::{BlockNumber, B256};

use eth::EthBlockHeader;
use revm::primitives::BlockEnv;

pub trait Hashable {
    /// Calculate the hash, this may be slow.
    fn hash_slow(&self) -> B256;
}

/// An EVM abstraction of a block header.
pub trait EvmBlockHeader: Hashable + AsAny {
    /// Returns the hash of the parent block's header.
    fn parent_hash(&self) -> &B256;
    /// Returns the block number.
    fn number(&self) -> BlockNumber;
    /// Returns the block timestamp.
    fn timestamp(&self) -> u64;
    /// Returns the state root hash.
    fn state_root(&self) -> &B256;
    /// Fills the EVM block environment with the header's data.
    fn fill_block_env(&self, blk_env: &mut BlockEnv);
}

pub enum BlockHeaderType {
    Eth(EthBlockHeader),
}

impl BlockHeaderType {
    pub fn to_box_dyn(&self) -> Box<dyn EvmBlockHeader> {
        match self {
            BlockHeaderType::Eth(header) => Box::new(header.clone()),
        }
    }
}

pub fn match_block_header_type(header: Box<dyn EvmBlockHeader>) -> Option<BlockHeaderType> {
    if let Some(eth_header) = header.as_ref().downcast_ref::<EthBlockHeader>() {
        Some(BlockHeaderType::Eth(eth_header.clone()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eth_block_header_to_box_dyn() {
        let eth_block_header = EthBlockHeader::default();

        let header_type = BlockHeaderType::Eth(eth_block_header.clone());

        let boxed_header: Box<dyn EvmBlockHeader> = header_type.to_box_dyn();
        assert!(boxed_header.as_ref().is::<EthBlockHeader>());
    }

    #[test]
    fn test_match_block_header_type() {
        let eth_block_header = EthBlockHeader::default();
        let header_type = BlockHeaderType::Eth(eth_block_header.clone());

        let boxed_header: Box<dyn EvmBlockHeader> = header_type.to_box_dyn();
        match match_block_header_type(boxed_header) {
            Some(BlockHeaderType::Eth(_)) => println!("Matched EthBlockHeader"),
            None => panic!("Did not match any block header type"),
        }
    }
}
