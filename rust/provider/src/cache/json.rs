use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use alloy_primitives::{Address, BlockNumber, Bytes, StorageKey, StorageValue, TxNumber, U256};
use anyhow::Context;
use block_header::EvmBlockHeader;
use ethers_core::types::BlockNumber as BlockTag;
use serde::{Deserialize, Serialize};

use super::EIP1186Proof;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub(crate) enum SerializableBlockTag {
    Number(u64),
    Latest,
}

impl From<BlockTag> for SerializableBlockTag {
    #[allow(clippy::panic)]
    fn from(ethers_block_number: BlockTag) -> Self {
        match ethers_block_number {
            BlockTag::Number(num) => SerializableBlockTag::Number(num.as_u64()),
            BlockTag::Latest => SerializableBlockTag::Latest,
            _ => panic!("Only specific block numbers are supported, got {ethers_block_number:?}"),
        }
    }
}

impl From<u64> for SerializableBlockTag {
    fn from(num: u64) -> Self {
        SerializableBlockTag::Number(num)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub(super) struct AccountQuery {
    pub(super) block_no: BlockNumber,
    pub(super) address: Address,
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub(super) struct BlockQuery {
    pub(super) block_no: SerializableBlockTag,
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub(super) struct ProofQuery {
    pub(super) block_no: BlockNumber,
    pub(super) address: Address,
    pub(super) storage_keys: BTreeSet<StorageKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub(super) struct StorageQuery {
    pub(super) block_no: BlockNumber,
    pub(super) address: Address,
    pub(super) key: StorageKey,
}

/// A simple JSON cache for storing responses from a provider.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct JsonCache {
    #[serde(skip)]
    file_path: Option<PathBuf>,

    #[serde(with = "ordered_map")]
    pub(super) partial_blocks: HashMap<BlockQuery, Option<Box<dyn EvmBlockHeader>>>,
    #[serde(with = "ordered_map")]
    pub(super) proofs: HashMap<ProofQuery, EIP1186Proof>,
    #[serde(with = "ordered_map")]
    pub(super) transaction_count: HashMap<AccountQuery, TxNumber>,
    #[serde(with = "ordered_map")]
    pub(super) balance: HashMap<AccountQuery, U256>,
    #[serde(with = "ordered_map")]
    pub(super) code: HashMap<AccountQuery, Bytes>,
    #[serde(with = "ordered_map")]
    pub(super) storage: HashMap<StorageQuery, StorageValue>,
}

impl PartialEq for JsonCache {
    fn eq(&self, other: &Self) -> bool {
        self.file_path == other.file_path
    }
}

impl JsonCache {
    /// Creates a new empty cache. It will be saved to the given file when dropped.
    pub(super) fn empty(file_path: PathBuf) -> Self {
        Self {
            file_path: Some(file_path),
            partial_blocks: HashMap::new(),
            proofs: HashMap::new(),
            transaction_count: HashMap::new(),
            balance: HashMap::new(),
            code: HashMap::new(),
            storage: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    /// Creates a new cache backed by the given file. It updates the file when dropped.
    pub(super) fn from_file(file_path: PathBuf) -> anyhow::Result<Self> {
        Self::load(&file_path).map(|mut cache| {
            cache.file_path = Some(file_path);
            cache
        })
    }

    /// Loads a cache from a file. Nothing is saved when the cache is dropped.
    pub(crate) fn load(file_path: &PathBuf) -> anyhow::Result<Self> {
        let file = File::open(file_path)
            .with_context(|| format!("failed to open cache file: {:?}", &file_path))?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).context("failed to deserialize cache")
    }

    /// Saves the cache to the file.
    fn save(&self) -> anyhow::Result<()> {
        if let Some(file_path) = &self.file_path {
            let file = File::create(file_path)
                .with_context(|| format!("failed to create cache file: {:?}", &file_path))?;
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, self).context("failed to serialize cache")?;
        }
        Ok(())
    }
}

impl Drop for JsonCache {
    #[allow(clippy::expect_used)]
    fn drop(&mut self) {
        self.save().expect("failed to save cache");
    }
}

/// A serde helper to serialize a HashMap into a vector sorted by key
mod ordered_map {
    use std::{collections::HashMap, hash::Hash};

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub(crate) fn serialize<S, K, V>(map: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        K: Ord + Serialize,
        V: Serialize,
    {
        let mut vec: Vec<(_, _)> = map.iter().collect();
        vec.sort_unstable_by_key(|&(k, _)| k);
        vec.serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D, K, V>(deserializer: D) -> Result<HashMap<K, V>, D::Error>
    where
        D: Deserializer<'de>,
        K: Eq + Hash + Deserialize<'de>,
        V: Deserialize<'de>,
    {
        let vec = Vec::<(_, _)>::deserialize(deserializer)?;
        Ok(vec.into_iter().collect())
    }
}
