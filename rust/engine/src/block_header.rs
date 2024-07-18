pub mod eth;

use std::any::TypeId;

use as_any::{AsAny, Downcast};

use alloy_primitives::{BlockNumber, B256};

use eth::EthBlockHeader;
use revm::primitives::BlockEnv;
use tracing::debug;

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

pub enum BlockHeader {
    Eth(EthBlockHeader),
}

impl From<BlockHeader> for Box<dyn EvmBlockHeader> {
    fn from(block_header: BlockHeader) -> Self {
        match block_header {
            BlockHeader::Eth(header) => Box::new(header),
        }
    }
}

impl TryFrom<Box<dyn EvmBlockHeader>> for BlockHeader {
    type Error = &'static str;

    fn try_from(header: Box<dyn EvmBlockHeader>) -> Result<Self, Self::Error> {
        if (*header).as_any().type_id() == TypeId::of::<EthBlockHeader>() {
            let casted_header = header.as_ref().downcast_ref::<EthBlockHeader>().unwrap();
            Ok(BlockHeader::Eth((*casted_header).clone()))
        } else {
            Err("Failed converting BlockHeader")
        }
    }
}

#[cfg(test)]
mod from_header_to_box_dyn {
    use super::*;

    #[test]
    fn eth() {
        let eth_block_header = EthBlockHeader::default();

        let header_type = BlockHeader::Eth(eth_block_header.clone());

        let boxed_header: Box<dyn EvmBlockHeader> = header_type.into();
        assert!(boxed_header.as_ref().is::<EthBlockHeader>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eth() {
        let eth_block_header = EthBlockHeader::default();

        let header: Box<dyn EvmBlockHeader> = Box::new(eth_block_header.clone());

        let result: Result<BlockHeader, _> = header.try_into();

        match result {
            Ok(BlockHeader::Eth(h)) => {
                assert_eq!(h.parent_hash, eth_block_header.parent_hash);
                assert_eq!(h.number, eth_block_header.number);
                assert_eq!(h.timestamp, eth_block_header.timestamp);
                assert_eq!(h.state_root, eth_block_header.state_root);
            }
            _ => panic!("Conversion to BlockHeader::Eth failed"),
        }
    }
}
