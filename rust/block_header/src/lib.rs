mod casting_utils;
mod eth;
mod forge;

use std::fmt::Debug;

use alloy_primitives::{B256, BlockNumber};
use alloy_rlp::Encodable;
use as_any::AsAny;
use auto_impl::auto_impl;
use casting_utils::is;
pub use common::Hashable;
use dyn_clone::{DynClone, clone_trait_object};
pub use eth::EthBlockHeader;
pub use forge::ForgeBlockHeader;
use revm::primitives::BlockEnv;
use serde::{Deserialize, Serialize};

/// An EVM abstraction of a block header.
#[auto_impl(Box, &)]
pub trait EvmBlockHeader: Hashable + Encodable + AsAny + Debug + DynClone + Send + Sync {
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

clone_trait_object!(EvmBlockHeader);

#[derive(Debug, Serialize, Deserialize)]
// We are not using #[serde(tag = "type", content = "data")] here because zkvm returns
// NotSupported error for it in deserialize_identifier function in deserializer.rs file
pub enum BlockHeader {
    Eth(EthBlockHeader),
    Forge(ForgeBlockHeader),
}

impl From<BlockHeader> for Box<dyn EvmBlockHeader> {
    fn from(block_header: BlockHeader) -> Self {
        match block_header {
            BlockHeader::Eth(header) => Box::new(header),
            BlockHeader::Forge(header) => Box::new(header),
        }
    }
}

impl TryFrom<&dyn EvmBlockHeader> for BlockHeader {
    type Error = &'static str;

    fn try_from(header: &dyn EvmBlockHeader) -> Result<Self, Self::Error> {
        if is::<EthBlockHeader>(header) {
            Ok(BlockHeader::Eth(header.try_into()?))
        } else if is::<ForgeBlockHeader>(header) {
            Ok(BlockHeader::Forge(header.try_into()?))
        } else {
            Err("Failed converting BlockHeader")
        }
    }
}

impl Serialize for Box<dyn EvmBlockHeader> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let block_header: BlockHeader = self
            .as_ref()
            .try_into()
            .map_err(serde::ser::Error::custom)?;
        block_header.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Box<dyn EvmBlockHeader> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let block_header = BlockHeader::deserialize(deserializer)?;
        let boxed: Box<dyn EvmBlockHeader> = block_header.into();
        Ok(boxed)
    }
}

#[cfg(feature = "test-utils")]
pub mod test_utils {
    use std::ops::RangeInclusive;

    use super::*;

    pub fn mock_block_header(number: BlockNumber, parent_hash: B256) -> Box<dyn EvmBlockHeader> {
        Box::new(EthBlockHeader {
            number,
            parent_hash,
            ..Default::default()
        })
    }

    pub fn mock_block_headers(blocks: RangeInclusive<BlockNumber>) -> Vec<Box<dyn EvmBlockHeader>> {
        let mut headers = vec![];
        let mut parent_hash = B256::default();

        for number in 0..*blocks.start() {
            parent_hash = mock_block_header(number, parent_hash).hash_slow();
        }

        for number in blocks {
            let header = mock_block_header(number, parent_hash);
            parent_hash = header.hash_slow();
            headers.push(header);
        }
        headers
    }
}

#[cfg(test)]
mod header_to_dyn_header {
    use as_any::Downcast;

    use super::*;

    #[test]
    fn eth() {
        let eth_block_header = EthBlockHeader::default();
        let header_type = BlockHeader::Eth(eth_block_header);
        let boxed_header: Box<dyn EvmBlockHeader> = header_type.into();

        assert!(boxed_header.as_ref().is::<EthBlockHeader>());
    }
}

#[cfg(test)]
mod dyn_header_to_header {
    use super::*;

    #[test]
    fn eth() {
        let eth_block_header = EthBlockHeader::default();
        let header: Box<dyn EvmBlockHeader> = Box::new(eth_block_header);

        // Perform the conversion using a reference to the trait object
        let result: Result<BlockHeader, _> = BlockHeader::try_from(header.as_ref());

        assert!(result.is_ok(), "Conversion to BlockHeader failed");
    }
}

#[cfg(test)]
mod serialize {
    use serde_json::to_string;
    use unsupported_block_header::UnsupportedBlockHeader;

    use super::*;

    #[test]
    fn success() -> anyhow::Result<()> {
        let header: Box<dyn EvmBlockHeader> = Box::new(EthBlockHeader::default());
        let serialized = to_string(&header)?;

        assert!(!serialized.is_empty());

        Ok(())
    }

    #[cfg(test)]
    mod unsupported_block_header {
        use alloy_primitives::{B256, keccak256};
        use alloy_rlp_derive::RlpEncodable;
        use revm::primitives::BlockEnv;

        use super::*;

        #[derive(Debug, Clone, RlpEncodable)]
        pub struct UnsupportedBlockHeader;

        impl Hashable for UnsupportedBlockHeader {
            fn hash_slow(&self) -> B256 {
                keccak256(alloy_rlp::encode(self))
            }
        }

        impl EvmBlockHeader for UnsupportedBlockHeader {
            fn parent_hash(&self) -> &B256 {
                unimplemented!()
            }

            fn number(&self) -> BlockNumber {
                unimplemented!()
            }

            fn timestamp(&self) -> u64 {
                unimplemented!()
            }

            fn state_root(&self) -> &B256 {
                unimplemented!()
            }

            fn fill_block_env(&self, _blk_env: &mut BlockEnv) {
                unimplemented!()
            }
        }
    }

    #[test]
    fn fail_with_unsupported_type() -> anyhow::Result<()> {
        let unsupported_header: Box<dyn EvmBlockHeader> = Box::new(UnsupportedBlockHeader);

        let result = to_string(&unsupported_header);

        if let Err(err) = result {
            let err_msg = err.to_string();
            assert_eq!(err_msg, "Failed converting BlockHeader");
        } else {
            panic!("Expected serialization to fail for unsupported type");
        }

        Ok(())
    }
}

#[cfg(test)]
mod deserialize {
    use std::fs;

    use alloy_primitives::hex;
    use serde_json::{self, Value, from_str, from_value};

    use super::*;

    const BLOCK_HEADER_INDEX: usize = 1;

    fn read_and_parse_json_file(file_path: &str) -> Result<Value, anyhow::Error> {
        let file_content = fs::read_to_string(file_path)
            .map_err(|e| anyhow::anyhow!("Failed to read the file {}: {}", file_path, e))?;
        let json_value = from_str(&file_content)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON from file {}: {}", file_path, e))?;
        Ok(json_value)
    }

    #[test]
    fn success() -> anyhow::Result<()> {
        let json_value = read_and_parse_json_file("testdata/mainnet_rpc_cache.json")?;
        let eth_block_header_json = &json_value["partial_blocks"][0][BLOCK_HEADER_INDEX]["Eth"];
        let deserialized_eth_header: EthBlockHeader = from_value(eth_block_header_json.clone())?;
        let expected_parent_hash = eth_block_header_json["parent_hash"].as_str().unwrap();
        let deserialized_parent_hash =
            hex::encode(deserialized_eth_header.parent_hash.as_ref() as &[u8]);

        assert_eq!(deserialized_parent_hash, expected_parent_hash.trim_start_matches("0x"));

        Ok(())
    }

    #[test]
    fn fail_with_invalid_data() -> anyhow::Result<()> {
        let json_value = read_and_parse_json_file("testdata/invalid_header.json")?;
        let eth_block_header_json = &json_value["partial_blocks"][0][BLOCK_HEADER_INDEX]["Eth"];
        let result: Result<EthBlockHeader, _> = from_value(eth_block_header_json.clone());

        if let Err(err) = result {
            let err_msg = err.to_string();
            assert_eq!(err_msg, "invalid type: null, expected struct EthBlockHeader");
        } else {
            panic!("Expected serialization to fail for unsupported type");
        }

        Ok(())
    }
}

#[cfg(test)]
mod serialize_and_deserialize {
    use lazy_static::lazy_static;
    use serde_json::{from_str, to_string};

    use super::*;

    lazy_static! {
        static ref ETH_BLOCK_HEADER: EthBlockHeader = EthBlockHeader::default();
    }

    #[test]
    fn correct_type() -> anyhow::Result<()> {
        let header: Box<dyn EvmBlockHeader> = Box::new(ETH_BLOCK_HEADER.clone());
        let serialized = to_string(&header)?;
        let deserialized: Box<dyn EvmBlockHeader> = from_str(&serialized)?;

        assert!(deserialized.as_ref().as_any().is::<EthBlockHeader>());

        Ok(())
    }

    #[test]
    fn correct_content() -> anyhow::Result<()> {
        let boxed_header: Box<dyn EvmBlockHeader> = Box::new(ETH_BLOCK_HEADER.clone());
        let serialized = to_string(&boxed_header)?;
        let deserialized: Box<dyn EvmBlockHeader> = from_str(&serialized)?;
        let deserialized_eth_header = deserialized
            .as_ref()
            .as_any()
            .downcast_ref::<EthBlockHeader>()
            .unwrap();
        assert_eq!(deserialized_eth_header, &EthBlockHeader::default());

        Ok(())
    }
}
