pub mod eth;

use std::any::TypeId;

// Downcast is needed to run is::<EthBlockHeader>() function
#[allow(unused_imports)]
use as_any::{AsAny, Downcast};

use alloy_primitives::{BlockNumber, B256};

use eth::EthBlockHeader;
use revm::primitives::BlockEnv;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait Hashable {
    /// Calculate the hash, this may be slow.
    fn hash_slow(&self) -> B256;
}

/// An EVM abstraction of a block header.
pub trait EvmBlockHeader: Hashable + AsAny + Debug {
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

#[derive(Debug, Serialize, Deserialize)]
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

impl TryFrom<&dyn EvmBlockHeader> for BlockHeader {
    type Error = &'static str;

    fn try_from(header: &dyn EvmBlockHeader) -> Result<Self, Self::Error> {
        if header.as_any().type_id() == TypeId::of::<EthBlockHeader>() {
            let eth_header = header
                .as_any()
                .downcast_ref::<EthBlockHeader>()
                .ok_or("Failed to downcast to EthBlockHeader")?
                .clone();
            Ok(BlockHeader::Eth(eth_header))
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

#[cfg(test)]
mod header_to_dyn_header {
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
    use super::*;
    use serde_json;

    #[test]
    fn success() {
        let eth_block_header = EthBlockHeader::default();
        let boxed_header: Box<dyn EvmBlockHeader> = Box::new(eth_block_header);
        let serialized = serde_json::to_string(&boxed_header).expect("Serialization failed");

        assert!(!serialized.is_empty());
    }

    #[test]
    fn fail_with_unsupported_type() {
        #[derive(Debug)]
        struct UnsupportedBlockHeader;

        impl Hashable for UnsupportedBlockHeader {
            fn hash_slow(&self) -> B256 {
                unimplemented!()
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

        let unsupported_header: Box<dyn EvmBlockHeader> = Box::new(UnsupportedBlockHeader);
        let result = serde_json::to_string(&unsupported_header);

        assert!(
            result.is_err(),
            "Expected serialization to fail for unsupported type"
        );
    }
}

#[cfg(test)]
mod deserialize {
    use super::*;
    use alloy_primitives::hex;
    use serde_json::{self, Value};
    use std::fs;

    fn read_and_parse_json_file(file_path: &str) -> Value {
        let file_content = fs::read_to_string(file_path).expect("Failed to read the file");
        let json_value: serde_json::Value =
            serde_json::from_str(&file_content).expect("Failed to parse JSON");
        json_value
    }

    #[test]
    fn success() {
        let json_value = read_and_parse_json_file("testdata/mainnet_rpc_cache.json");
        let eth_block_header_json = &json_value["partial_blocks"][0][1]["Eth"];
        let deserialized_eth_header: EthBlockHeader =
            serde_json::from_value(eth_block_header_json.clone()).expect("Deserialization failed");
        let expected_parent_hash = eth_block_header_json["parent_hash"].as_str().unwrap();
        let deserialized_parent_hash =
            hex::encode(deserialized_eth_header.parent_hash.as_ref() as &[u8]);

        assert_eq!(
            deserialized_parent_hash,
            expected_parent_hash.trim_start_matches("0x")
        );
    }

    #[test]
    fn fail_with_invalid_data() {
        let json_value = read_and_parse_json_file("testdata/invalid_header.json");
        let eth_block_header_json = &json_value["partial_blocks"][0][1]["Eth"];
        let result: Result<EthBlockHeader, _> =
            serde_json::from_value(eth_block_header_json.clone());

        assert!(
            result.is_err(),
            "Expected deserialization to fail due to invalid parent hash"
        );
    }
}

#[cfg(test)]
mod serialize_and_deserialize {
    use super::*;
    use serde_json;

    fn serialize_and_deserialize_eth_block_header() -> Box<dyn EvmBlockHeader> {
        let eth_block_header = EthBlockHeader::default();
        let boxed_header: Box<dyn EvmBlockHeader> = Box::new(eth_block_header);
        let serialized = serde_json::to_string(&boxed_header).expect("Serialization failed");
        let deserialized: Box<dyn EvmBlockHeader> =
            serde_json::from_str(&serialized).expect("Deserialization failed");

        deserialized
    }

    #[test]
    fn correct_type() {
        let deserialized = serialize_and_deserialize_eth_block_header();

        assert!(deserialized.as_ref().as_any().is::<EthBlockHeader>());
    }

    #[test]
    fn correct_content() {
        let deserialized = serialize_and_deserialize_eth_block_header();
        let deserialized_eth_header = deserialized
            .as_ref()
            .as_any()
            .downcast_ref::<EthBlockHeader>()
            .unwrap();

        assert_eq!(deserialized_eth_header, &EthBlockHeader::default());
    }
}
